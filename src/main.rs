#![allow(dead_code, unused_imports)]

use std::path::{PathBuf, Path};
use std::sync::mpsc;

mod render;
mod edit;
mod config;
mod wrap;

use render::GuiRenderer;
use edit::{Editor, Mode};
use edit::event::EventData;
use wrap::*;
use config::{configure, Config};

use macroquad::prelude::*;
use macroquad::miniquad::window::set_window_size;



const COLOR_BG:  Color = Color::from_rgba(40, 43, 46, 255);
const FRAMERATE: f32 = 20.;




#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("macroquad error")]
    Macroquad(#[from] macroquad::Error),
}

type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
struct Application {
    ed: Editor,
    // TODO: this is kinda expensive, replace with VecDeque
    event_queue: mpsc::Receiver<EventData>,
    config: Config,
    should_quit: bool,
    renderer: GuiRenderer,
}

impl Application {

    pub async fn new(path: Option<impl AsRef<Path>>) -> AppResult<Self> {

        let (tx, rx) = mpsc::channel();

        let ed = if let Some(path) = path {
            Editor::with_file(tx, path)?
        } else {
            Editor::new(tx)
        };

        let mut self_ = Self {
            event_queue:  rx,
            should_quit:  false,
            renderer:     GuiRenderer::new().await?,
            config:       Config::default(),
            ed,
        };

        configure(&mut self_);
        Ok(self_)

    }


    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    fn handle_input(&mut self) {

        let found_bind = self.dispatch_keybinds();

        if self.ed.mode() == Mode::Insert && !found_bind {

            if let Some(c) = get_char_pressed() {
                let buf = self.ed.buf_mut().unwrap();
                buf.insert_char(c);
                buf.move_right();
            }

        } else {
            clear_input_queue();
        }

    }

    fn handle_events(&mut self) {

        if let Ok(ref ev) = self.event_queue.try_recv() {
            if let Some(callback) = self.config.autocmds().get(&ev.base()) {
                callback(self, ev);
            }
        }

    }

    /// Returns false if no keybindings have been dispatched.
    fn dispatch_keybinds(&mut self) -> bool {

        let mut found_bind = false;
        for (bind, action) in self.config.clone().keybinds() {

            if bind.mode != self.ed.mode() { continue }

            if bind.key.is_active() {
                action(self);
                found_bind = true;
            }

        }

        found_bind

    }

    fn render(&mut self) {

        clear_background(COLOR_BG);

        let status = self.config.statusline()(self);
        let bounds = Rect::new(0., 0., screen_width(), screen_height());
        self.renderer.render(bounds, &self.ed, &status);
    }

}

fn parse_args() -> clap::ArgMatches {
    use clap::arg;

    clap::command!()
        .arg(arg!([filename] "filename"))
        .get_matches()
}

#[macroquad::main("main")]
async fn main() -> AppResult<()> {

    let matches = parse_args();

    let path = matches
        .get_one::<String>("filename")
        .map(PathBuf::from);

    let mut app = Application::new(path).await?;

    set_window_size(1600, 900);

    'running: loop {

        if app.should_quit {
            break 'running;
        }

        app.handle_events();
        app.handle_input();
        app.render();

        sleep_framerate(FRAMERATE);
        next_frame().await;

    }

    Ok(())
}
