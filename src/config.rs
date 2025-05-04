use std::collections::HashMap;
use std::hash::Hash;
use crate::edit::Statusline;

use macroquad::prelude::*;

use crate::edit::{Editor, Mode};
use crate::edit::event::{EventData, Event};
use crate::keys::{Keybind, Key, Modifiers};
use crate::{Application, keybind};




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



pub fn configure(app: &mut Application) {

    app.config.set_status(|app| {
        let ed = &app.ed;

        let mode = ed.mode().to_string();
        let buf_count = ed.buffers().count();
        let win = ed.windows().idcount;
        let win_count = ed.windows().count();

        if let Some(buf) = ed.buf() {
            let filename = match buf.filename.as_ref() {
                Some(path) => path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap(),
                None => "<mem>",
            };
            let line      = buf.cur.y + 1;
            let char      = buf.cur.x + 1;
            let append    = if buf.append { "[A]" } else { "[_]" };
            let linecount = buf.getlines().len();
            Statusline::new(
                format!("{mode} {append} | {filename}"),
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
    app.config.keymap(keybind!(Normal, I, NoMod), |app| app.ed.set_mode(Mode::Insert));
    app.config.keymap(keybind!(Normal, I, Shift), |app| {
        app.ed.buf_mut().unwrap().move_start_line();
        app.ed.set_mode(Mode::Insert);
    });

    app.config.keymap(keybind!(Normal, Q, NoMod), |app| app.quit());

    app.config.keymap(keybind!(Insert, Backspace, NoMod), |app| {
        app.ed.buf_mut().unwrap().delete_char_before();
    });

    app.config.keymap(keybind!(Insert, Escape, NoMod), |app| {
        app.ed.set_mode(Mode::Normal);
        app.ed.buf_mut().unwrap().move_left();
    });

        //
        //     keybind!(Normal, X,    NoMod, |ed| ed.buf_mut().unwrap().delete_char()),
        //     keybind!(Normal, Key0, NoMod, |ed| ed.buf_mut().unwrap().move_start_line()),
        //     keybind!(Normal, Key4, Shift, |ed| ed.buf_mut().unwrap().move_end_line()),
        //
        //     keybind!(Normal, W, Shift, |ed| { ed.windows_mut().add(None); }),
        //     keybind!(Normal, U, Shift, |ed| ed.windows_mut().current_mut().unwrap().use_buf(0)),
        //     keybind!(Normal, X, Shift, |ed| ed.windows_mut().delete()),
        //     keybind!(Normal, N, Shift, |ed| ed.windows_mut().next(true)),
        //     keybind!(Normal, P, Shift, |ed| ed.windows_mut().prev(true)),
        //
        //
        //     keybind!(Normal, Z, Shift, |ed| { ed.buffers_mut().add(); }),
        //
        //
        //     keybind!(Normal, A, NoMod, |ed| {
        //         ed.buf_mut().unwrap().move_right();
        //         ed.set_mode(Mode::Insert);
        //     }),
        //
        //
        //     keybind!(Normal, A, Shift, |ed| {
        //         ed.buf_mut().unwrap().move_append_end_line();
        //         ed.set_mode(Mode::Insert);
        //     }),
        //
        //
        //     keybind!(Normal, O, Shift, |ed| {
        //         ed.buf_mut().unwrap().newline_above();
        //         ed.set_mode(Mode::Insert);
        //     }),
        //
        //     keybind!(Normal, O, NoMod, |ed| {
        //         ed.buf_mut().unwrap().newline_below();
        //         ed.buf_mut().unwrap().move_down();
        //         ed.set_mode(Mode::Insert);
        //     }),
        //
        //     // keybind!(Normal, R, NoMod, |ed| ed.run_line()),
        //
        //
        //     keybind!(Insert, U, Ctrl, |ed| ed.buf_mut().unwrap().clear_current_line()),
        //     keybind!(Insert, Enter, NoMod, |ed| ed.buf_mut().unwrap().split_newline()),
        //     keybind!(Insert, Tab, NoMod, |ed| {
        //         // TODO: appending tab at end of line not working correctly
        //         let buf = ed.buf_mut().unwrap();
        //         buf.insert_string("    ");
        //         buf.move_right();
        //         buf.move_right();
        //         buf.move_right();
        //         buf.move_right();
        //     }),





}
