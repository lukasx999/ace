use std::collections::BTreeMap;

use super::buffer::BufferID;



/// Each window has an identifier thats unique in one editor session.
pub type WindowID = usize;


#[derive(Debug, Clone, Copy, Default)]
pub struct Window {
    pub bufid: Option<BufferID>,
}

impl Window {

    pub fn new(buf: BufferID) -> Self {
        Self { bufid: Some(buf) }
    }

    pub fn use_buf(&mut self, id: BufferID) {
        self.bufid = Some(id);
    }

}

#[derive(Debug, Clone, Default)]
pub struct Windows {
    pub current: WindowID,
    pub idcount: WindowID,
    /// Using btreemap, as the data should always be in order.
    pub windows: BTreeMap<WindowID, Window>,
}

impl Windows {

    pub fn with_buffer(id: BufferID) -> Self {
        Self {
            windows: BTreeMap::from([ (0, Window::new(id)) ]),
            idcount: 1,
            current: 0,
        }
    }

    #[must_use]
    pub fn winid(&self) -> Option<WindowID> {
        if self.windows.is_empty() {
            None
        } else {
            Some(self.current)
        }
    }

    #[must_use]
    pub fn get(&self, id: WindowID) -> Option<&Window> {
        self.windows.get(&id)
    }

    #[must_use]
    pub fn get_mut(&mut self, id: WindowID) -> Option<&mut Window> {
        self.windows.get_mut(&id)
    }

    #[must_use]
    pub fn current(&self) -> Option<&Window> {
        self.windows.get(&self.winid()?)
    }

    #[must_use]
    pub fn current_mut(&mut self) -> Option<&mut Window> {
        self.windows.get_mut(&self.winid()?)
    }

    pub fn add(&mut self, id: Option<BufferID>) -> WindowID {
        let win = if let Some(id) = id {
            Window::new(id)
        } else {
            Window::default()
        };

        let id = self.idcount;
        self.idcount += 1;
        let ret = self.windows.insert(id, win);
        assert!(ret.is_none());
        id
    }

    pub fn delete(&mut self) {
        if self.windows.is_empty() { return }

        self.windows.remove(&self.current);

        let max = self.windows
            .len()
            .saturating_sub(1);
        self.idcount = self.idcount.clamp(0, max);
    }

    pub fn next(&mut self, wrap: bool) {

        self.current += 1;
        let len = self.windows.len();

        // BUG: switch to hashmap
        if self.current >= len {
            self.current = if wrap { 0 } else { len - 1 };
        }
    }

    pub fn prev(&mut self, wrap: bool) {
        // BUG: switch to hashmap
        self.current = self.current
            .checked_sub(1)
            .unwrap_or(if wrap { self.windows.len() - 1 } else { 0 });
    }

    #[must_use]
    pub fn count(&self) -> usize {
        self.windows.len()
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_windows() {
        let mut wins = Windows::default();
        wins.add(None);
        wins.add(None);
        wins.add(None);
        assert_eq!(wins.count(), 3);

        wins.next(true);
        wins.next(true);
        wins.next(true);
        assert_eq!(wins.current, 0);

        wins.prev(true);
        assert_eq!(wins.current, 2);

        wins.next(false);
        assert_eq!(wins.current, 2);

        wins.prev(false);
        wins.prev(false);
        wins.prev(false);
        assert_eq!(wins.current, 0);
    }

}
