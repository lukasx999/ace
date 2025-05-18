use std::{fmt::Display, sync::mpsc::Sender};
use std::io;
use std::path::Path;
use std::collections::VecDeque;
use std::cell::RefCell;
use std::sync::Mutex;

pub mod buffer;
pub mod window;
pub mod event;
use event::EventData;

use buffer::{Buffer, Buffers, BufferID};
use window::{Windows, WindowID, Window};

pub struct EditorContext {
    pub event_queue: VecDeque<EventData>,
}

impl EditorContext {
    pub const fn new() -> Self {
        Self {
            event_queue: VecDeque::new(),
        }
    }
}

pub static CONTEXT: Mutex<EditorContext> = Mutex::new(EditorContext::new());


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

pub type Message = String;

#[derive(Debug, Clone)]
pub struct Editor {
    messages: Vec<Message>,
    buffers:  Buffers,
    mode:     Mode,
    windows:  Windows,
}

impl Editor {

    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            windows:  Windows::default(),
            buffers:  Buffers::default(),
            mode:     Mode::default(),
        }
    }

    pub fn with_file(path: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self {
            buffers: Buffers::with_file(path)?,
            windows: Windows::with_buffer(0),
            ..Self::new()
        })
    }

    /// Add a [`Message`] to the Editors message buffer.
    pub fn add_message(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    /// Dump all [`Message`]s into a newly created [`Buffer`], which is contained
    /// in a newly created [`Window`]
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

    /// Retrieves the [`WindowID`] of the currently focused [`Window`]
    #[must_use]
    pub fn winid(&self) -> Option<WindowID> {
        self.windows().winid()
    }

    #[must_use]
    /// Retrieves the [`BufferID`] of the currently focused [`Buffer`]
    pub fn bufid(&self) -> Option<BufferID> {
        let id = self.windows().winid()?;
        self.windows.get(id)?.buf()
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

}
