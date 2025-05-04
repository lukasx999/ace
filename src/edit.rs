use std::{fmt::Display, sync::mpsc::Sender};
use std::io;
use std::path::Path;

pub mod buffer;
pub mod window;
pub mod event;
use event::EventData;

use buffer::{Buffer, Buffers, BufferID};
use window::{Windows, WindowID};



#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum Mode {
    #[default] Normal,
    Insert,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Mode::Normal => "normal",
            Mode::Insert => "insert",
        })
    }
}




#[derive(Debug, Clone, Default)]
pub struct Statusline {
    pub left:   String,
    pub center: String,
    pub right:  String,
}

impl Statusline {
    pub fn new(left: String, center: String, right: String) -> Self {
        Self { left, center, right }
    }
}


#[derive(Debug, Clone)]
pub struct Editor {
    event_tx: Sender<EventData>,
    messages: Vec<String>,
    buffers:  Buffers,
    mode:     Mode,
    windows:  Windows,
}

impl Editor {

    pub fn new(event_tx: Sender<EventData>) -> Self {
        Self {
            event_tx,
            messages: Vec::new(),
            windows:  Windows::default(),
            buffers:  Buffers::default(),
            mode:     Mode::default(),
        }
    }

    pub fn with_file(event_tx: Sender<EventData>, path: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self {
            buffers: Buffers::with_file(path)?,
            windows: Windows::with_buffer(0),
            ..Self::new(event_tx)
        })
    }

    pub fn add_message(&mut self, str: String) {
        self.messages.push(str);
    }

    pub fn show_messages(&mut self) {
        let id = self.buffers.add();
        self.windows.add(Some(id));

        let messages = self.messages.clone();
        self
            .buffers_mut()
            .get_mut(id)
            .unwrap()
            .load_buffer(messages);
    }

    #[must_use]
    pub fn winid(&self) -> Option<WindowID> {
        self.windows().winid()
    }

    #[must_use]
    pub fn bufid(&self) -> Option<BufferID> {
        self.windows().current()?.buf
    }

    #[must_use]
    pub fn buf(&self) -> Option<&Buffer> {
        let id = self.bufid()?;
        self.buffers().get(id)
    }

    #[must_use]
    pub fn buf_mut(&mut self) -> Option<&mut Buffer> {
        let id = self.bufid()?;
        self.buffers_mut().get_mut(id)
    }

    #[must_use]
    pub fn windows(&self) -> &Windows {
        &self.windows
    }

    #[must_use]
    pub fn windows_mut(&mut self) -> &mut Windows {
        &mut self.windows
    }

    #[must_use]
    pub fn buffers(&self) -> &Buffers {
        &self.buffers
    }

    #[must_use]
    pub fn buffers_mut(&mut self) -> &mut Buffers {
        &mut self.buffers
    }

    #[must_use]
    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.event_tx
            .send(EventData::ModeChanged(mode))
            .unwrap();
    }

}
