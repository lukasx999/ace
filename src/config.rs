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

#[derive(Clone)]
pub struct Config {
    // TODO: repeat rate
    // TODO: use hashmap instead of vec
    pub keybinds: Vec<(Keybind, Action)>,
    pub autocmds: HashMap<Event, Autocmd>,
    pub statusline: StatuslineCallback,
}


impl Default for Config {
    fn default() -> Self {
        Self {
            autocmds: HashMap::default(),
            keybinds: Vec::default(),
            statusline: |_| { Statusline::default() },
        }
    }
}


pub fn configure(app: &mut Application) {

    app.set_status(|app| {
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

    app.autocmd(Event::BufDel, |_app, data| {
        dbg!(data);
    });
    app.autocmd(Event::BufNew, |_app, data| {
        dbg!(data);
    });

    app.keymap(keybind!(Normal, J, NoMod), |app| app.ed.buf_mut().unwrap().move_down());
    app.keymap(keybind!(Normal, K, NoMod), |app| app.ed.buf_mut().unwrap().move_up());
    app.keymap(keybind!(Normal, L, NoMod), |app| app.ed.buf_mut().unwrap().move_right());
    app.keymap(keybind!(Normal, H, NoMod), |app| app.ed.buf_mut().unwrap().move_left());

    app.keymap(keybind!(Normal, D, Ctrl),  |app| app.ed.buf_mut().unwrap().move_down_many(10));
    app.keymap(keybind!(Normal, U, Ctrl),  |app| app.ed.buf_mut().unwrap().move_up_many(10));
    app.keymap(keybind!(Normal, D, NoMod), |app| app.ed.buf_mut().unwrap().delete_line());
    app.keymap(keybind!(Normal, M, NoMod), |app| app.ed.show_messages());
    app.keymap(keybind!(Normal, G, Shift), |app| app.ed.buf_mut().unwrap().move_bottom());
    app.keymap(keybind!(Normal, G, NoMod), |app| app.ed.buf_mut().unwrap().move_top());
    app.keymap(keybind!(Normal, I, NoMod), |app| app.ed.set_mode(Mode::Insert));
        //     keybind!(Normal, I, Shift, |ed| {
        //         ed.buf_mut().unwrap().move_start_line();
        //         ed.set_mode(Mode::Insert);
        //     }),
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
        //     // TODO:
        //     // keybind!(Normal, Q, NoMod, |ed| {
        //     // }),
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
        //     keybind!(Insert, Backspace, NoMod, |ed| {
        //
        //         ed.buf_mut().unwrap().delete_char_before();
        //
        //         // ed.buf_mut().unwrap().move_left();
        //         // ed.buf_mut().unwrap().delete_char();
        //     }),
        //     keybind!(Insert, Escape, NoMod, |ed| {
        //         ed.set_mode(Mode::Normal);
        //         ed.buf_mut().unwrap().move_left();
        //     }),
        //
        // ];



}
