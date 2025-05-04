use std::collections::HashMap;
use std::hash::Hash;
use crate::edit::Statusline;

use macroquad::prelude::*;

use crate::edit::{Editor, Event, EventData, Mode};
use crate::keys::{Keybind, Key, Modifiers};
use crate::{Application, keybind};




pub type StatuslineCallback = Box<dyn FnMut(&Editor) -> Statusline>;
pub type Action = Box<dyn FnMut(&mut Editor)>;
pub type Autocmd = Box<dyn FnMut(&mut Editor, &EventData)>;

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
            statusline: Box::new(|_| { Statusline::default() }),
        }
    }

}


pub fn configure(app: &mut Application) {

    app.set_status(Box::new(|ed| {

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
    }));


    app.autocmd(Event::BufNew, Box::new(|_ed, data| { dbg!(data); }));
    app.autocmd(Event::BufDel, Box::new(|_ed, data| { dbg!(data); }));

    app.keymap(keybind!(Normal, J, NoMod), Box::new(|ed| ed.buf_mut().unwrap().move_down()));
    app.keymap(keybind!(Normal, K, NoMod), Box::new(|ed| ed.buf_mut().unwrap().move_up()));
    app.keymap(keybind!(Normal, L, NoMod), Box::new(|ed| ed.buf_mut().unwrap().move_right()));
    app.keymap(keybind!(Normal, H, NoMod), Box::new(|ed| ed.buf_mut().unwrap().move_left()));

    app.keymap(keybind!(Normal, D, Ctrl), Box::new(|ed| ed.buf_mut().unwrap().move_down_many(10)));
    app.keymap(keybind!(Normal, U, Ctrl), Box::new(|ed| ed.buf_mut().unwrap().move_up_many(10)));

    app.keymap(keybind!(Normal, D, NoMod), Box::new(|ed| ed.buf_mut().unwrap().delete_line()));

    app.keymap(keybind!(Normal, M, NoMod), Box::new(|ed| ed.show_messages()));

    app.keymap(keybind!(Normal, G, Shift), Box::new(|ed| ed.buf_mut().unwrap().move_bottom()));
    app.keymap(keybind!(Normal, G, NoMod), Box::new(|ed| ed.buf_mut().unwrap().move_top()));

    app.keymap(keybind!(Normal, I, NoMod), Box::new(|ed| ed.set_mode(Mode::Insert)));
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
