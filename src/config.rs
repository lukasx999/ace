use std::collections::HashMap;
use crate::edit::Statusline;

use macroquad::prelude::*;

use crate::edit::Mode;
use crate::edit::event::{EventData, Event};
use crate::{Application, keybind};

mod keys;
use keys::{Keybind, Key, Modifiers};



pub type StatuslineCallback = fn(&Application) -> Statusline;
pub type Action = fn(&mut Application);
pub type Autocmd = fn(&mut Application, &EventData);

#[derive(Debug, Clone)]
pub struct Config {
    keybinds: HashMap<Keybind, Action>,
    autocmds: HashMap<Event, Autocmd>,
    statusline: StatuslineCallback,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            autocmds: HashMap::default(),
            keybinds: HashMap::default(),
            statusline: |_| Statusline::default(),
        }
    }
}

impl Config {

    pub fn set_status(&mut self, action: StatuslineCallback) {
        self.statusline = action;
    }

    pub fn keymap(&mut self, keybind: Keybind, action: Action) {
        self.keybinds.insert(keybind, action);
    }

    pub fn autocmd(&mut self, ev: Event, action: Autocmd) {
        self.autocmds.insert(ev, action);
    }

    #[must_use]
    pub fn autocmds(&self) -> &HashMap<Event, Autocmd> {
        &self.autocmds
    }

    #[must_use]
    pub fn statusline(&self) -> fn(&Application) -> Statusline {
        self.statusline
    }

    #[must_use]
    pub fn keybinds(&self) -> &HashMap<Keybind, Action> {
        &self.keybinds
    }

}


// TODO: refactor into function
macro_rules! buf {
    ($app:expr) => {
        $app.ed.buf_mut().unwrap()
    }
}


pub fn configure(app: &mut Application) {

    app.config.set_status(|app| {
        let ed = &app.ed;

        let mode = ed.buf().unwrap().mode().to_string();
        let buf_count = ed.buffers().count();
        let win = ed.windows().idcount;
        let win_count = ed.windows().count();

        if let Some(buf) = ed.buf() {
            let filename = match buf.filename().as_ref() {
                Some(path) => path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap(),
                None => "<mem>",
            };
            let line         = buf.cursor().y + 1;
            let char         = buf.cursor().x + 1;
            let append       = if buf.append { "[A]" } else { "[_]" };
            let linecount    = buf.getlines().len();
            let search_query = &buf.search_query;
            let search_count = buf.search().len();
            let clipboard    = buf.clipboard.len();
            Statusline::new(
                format!("{mode} {append} | {filename} | {search_query} ({search_count}) | Clipboard: {clipboard}"),
                format!("{linecount} Lines | {line}:{char}"),
                format!("{buf_count} Buffers | {win}/{win_count} Windows")
            )
        } else {
            Statusline::new(
                mode,
                format!("{buf_count} Buffers"),
                format!("{win}/{win_count} Windows")
            )
        }
    });

    app.config.autocmd(Event::BufDel, |_app, data| {
        dbg!(data);
    });
    app.config.autocmd(Event::BufNew, |_app, data| {
        dbg!(data);
    });

    app.config.keymap(keybind!(Normal, P, Shift), |app| app.ed.buf_mut().unwrap().paste());
    app.config.keymap(keybind!(Normal, P, NoMod), |app| app.ed.buf_mut().unwrap().paste_pop());
    app.config.keymap(keybind!(Normal, Y, NoMod), |app| app.ed.buf_mut().unwrap().yank_line());
    app.config.keymap(keybind!(Normal, J, NoMod), |app| app.ed.buf_mut().unwrap().move_down());
    app.config.keymap(keybind!(Normal, K, NoMod), |app| app.ed.buf_mut().unwrap().move_up());
    app.config.keymap(keybind!(Normal, L, NoMod), |app| app.ed.buf_mut().unwrap().move_right());
    app.config.keymap(keybind!(Normal, H, NoMod), |app| app.ed.buf_mut().unwrap().move_left());

    app.config.keymap(keybind!(Normal, D, Ctrl),  |app| app.ed.buf_mut().unwrap().move_down_many(10));
    app.config.keymap(keybind!(Normal, U, Ctrl),  |app| app.ed.buf_mut().unwrap().move_up_many(10));
    app.config.keymap(keybind!(Normal, D, NoMod), |app| app.ed.buf_mut().unwrap().delete_line());
    app.config.keymap(keybind!(Normal, M, NoMod), |app| app.ed.show_messages());
    app.config.keymap(keybind!(Normal, G, Shift), |app| app.ed.buf_mut().unwrap().move_bottom());
    app.config.keymap(keybind!(Normal, G, NoMod), |app| app.ed.buf_mut().unwrap().move_top());
    app.config.keymap(keybind!(Normal, I, NoMod), |app| app.ed.buf_mut().unwrap().set_mode(Mode::Insert));
    app.config.keymap(keybind!(Normal, I, Shift), |app| {
        app.ed.buf_mut().unwrap().move_start_line();
        app.ed.buf_mut().unwrap().set_mode(Mode::Insert);
    });

    app.config.keymap(keybind!(Normal, Q, NoMod), |app| app.quit());

    app.config.keymap(keybind!(Insert, Backspace, NoMod), |app| {
        let buf = buf!(app);
        let append = buf.append;

        buf.move_left();
        buf.delete_char();

        if append {
            buf.move_append_end_line();
        }
    });

    app.config.keymap(keybind!(Insert, Escape, NoMod), |app| {
        app.ed.buf_mut().unwrap().set_mode(Mode::Normal);
        app.ed.buf_mut().unwrap().move_left();
    });

    app.config.keymap(keybind!(Normal, Period, NoMod), |app| {
        let buf = &mut app.renderer.canvas.win.buf;
        buf.set_fontsize(buf.fontsize() + 1);
    });

    app.config.keymap(keybind!(Normal, Comma, NoMod), |app| {
        let buf = &mut app.renderer.canvas.win.buf;
        buf.set_fontsize(buf.fontsize() - 1);
    });


    app.config.keymap(keybind!(Normal, X,    NoMod), |app| buf!(app).delete_char());
    app.config.keymap(keybind!(Normal, Key0, NoMod), |app| buf!(app).move_start_line());
    app.config.keymap(keybind!(Normal, Key4, Shift), |app| buf!(app).move_end_line());
    app.config.keymap(keybind!(Normal, W, Shift), |app| { app.ed.windows_mut().add(None); });
    app.config.keymap(keybind!(Normal, X, Shift), |app| app.ed.windows_mut().delete());
    // app.config.keymap(keybind!(Normal, N, Shift), |app| app.ed.windows_mut().next(true));
    // app.config.keymap(keybind!(Normal, P, Shift), |app| app.ed.windows_mut().prev(true));
    app.config.keymap(keybind!(Normal, Z, Shift), |app| { app.ed.buffers_mut().add(); });
    app.config.keymap(keybind!(Normal, A, NoMod), |app| {
        let buf = buf!(app);
        // not working correctly when on last char
        buf.append = true;
        buf.move_right();
        app.ed.buf_mut().unwrap().set_mode(Mode::Insert);
    });
    app.config.keymap(keybind!(Normal, A, Shift), |app| { buf!(app).move_append_end_line(); app.ed.buf_mut().unwrap().set_mode(Mode::Insert); });
    app.config.keymap(keybind!(Normal, O, Shift), |app| { buf!(app).newline_above(); app.ed.buf_mut().unwrap().set_mode(Mode::Insert); });
    app.config.keymap(keybind!(Normal, O, NoMod), |app| {
        buf!(app).newline_below();
        buf!(app).move_down();
        app.ed.buf_mut().unwrap().set_mode(Mode::Insert);
    });
    app.config.keymap(keybind!(Insert, U, Ctrl), |app| buf!(app).clear_current_line());
    app.config.keymap(keybind!(Insert, Enter, NoMod), |app| buf!(app).split_newline());

    app.config.keymap(keybind!(Normal, U, Shift), |app| {
        let id = app.ed.winid().unwrap();
        let win = app.ed.windows_mut().get_mut(id).unwrap();
        win.set_buf(0);
    });

    app.config.keymap(keybind!(Insert, Tab, NoMod), |app| {
        let buf = buf!(app);
        buf.insert_string("    ");
        buf.move_right();
        buf.move_right();
        buf.move_right();
        buf.move_right();
    });

}
