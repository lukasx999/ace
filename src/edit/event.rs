use super::{BufferID, WindowID, Mode};

/// This gets passed to the subscribers of the event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventData {
    Init,
    Deinit,
    WinNew(WindowID),
    WinDel(WindowID),
    BufNew(BufferID),
    BufDel(BufferID),
    ModeChanged(Mode),
}

// TODO: procmacro for this madness
impl EventData {
    pub fn base(&self) -> Event {
        match self {
            Self::Init           => Event::Init,
            Self::Deinit         => Event::Deinit,
            Self::WinNew(_)      => Event::WinNew,
            Self::WinDel(_)      => Event::WinDel,
            Self::BufNew(_)      => Event::BufNew,
            Self::BufDel(_)      => Event::BufDel,
            Self::ModeChanged(_) => Event::ModeChanged,
        }
    }
}

/// Used for subscribing to certain events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    Init,
    Deinit,
    WinNew,
    WinDel,
    BufNew,
    BufDel,
    ModeChanged,
}
