use std::io::Write;
use std::{fs, io, path};
use std::path::{PathBuf, Path};
use std::collections::BTreeMap;



/// Each buffer has an identifier which is unique in one editor session.
pub type BufferID = usize;



#[derive(Debug, Clone, Default)]
pub struct Buffers {
    idcount: BufferID,
    /// Using [`BTreeMap`], as the data should always be in order.
    buffers: BTreeMap<BufferID, Buffer>,
}

impl Buffers {

    pub fn with_file(path: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self {
            buffers: BTreeMap::from([ (0, Buffer::with_file(path)?) ]),
            idcount: 1
        })
    }

    #[must_use]
    pub fn get(&self, id: BufferID) -> Option<&Buffer> {
        self.buffers.get(&id)
    }

    #[must_use]
    pub fn get_mut(&mut self, id: BufferID) -> Option<&mut Buffer> {
        self.buffers.get_mut(&id)
    }

    #[must_use]
    pub fn count(&self) -> usize {
        self.buffers.len()
    }

    pub fn add(&mut self) -> BufferID {
        let id = self.idcount;
        self.idcount += 1;
        let ret = self.buffers.insert(id, Buffer::new());
        assert!(ret.is_none());
        id
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Cursor {
    // cursor must be signed for out-of-bounds checking
    pub x: isize,
    pub y: isize,
}

impl Cursor {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl From<(isize, isize)> for Cursor {
    fn from(value: (isize, isize)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl std::ops::Sub for Cursor {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Cursor::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Add for Cursor {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Cursor::new(self.x + rhs.x, self.y + rhs.y)
    }
}


// TODO:
#[derive(Debug, Clone)]
pub struct Selection {
    target: Cursor,
}

// TODO: implement kill ring and permanent clipboard

#[derive(Debug, Clone)]
pub struct Buffer {
    filename: Option<PathBuf>,
    cursor: Cursor,
    lines: Vec<String>,

    pub search_query: String,

    /// allows the cursor to be out-of-bounds
    /// by one char at the end of the line.
    /// used by `A`, and `a` at end of a line.
    // TODO: make private, as its an implementation detail
    pub append: bool,
}

impl Buffer {

    //
    // Buffer Management
    //

    pub fn with_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let mut self_ = Self::new();
        self_.load_file(path)?;
        Ok(self_)
    }

    pub fn new() -> Self {
        Self {
            search_query: "foo".to_string(),
            filename: None,
            cursor: Cursor::default(),
            lines: vec![ String::new() ],
            append: true,
        }
    }

    pub fn set_filename(&mut self, filename: impl AsRef<Path>) -> io::Result<()> {
        // using absolute() instead of canonicalize() as
        // `filename` could possibly not exist yet
        self.filename = Some(path::absolute(filename)?);
        Ok(())
    }

    /// Wipes the buffer, loading the given buffer.
    pub fn load_buffer(&mut self, buf: Vec<String>) {
        *self = Self::new();

        self.lines = buf;

        let is_empty = self.lines.is_empty();
        self.append = is_empty;

        if is_empty {
            self.lines.push(String::new());
        }

    }

    /// Wipes the buffer, loading a buffer from the file at the given path.
    pub fn load_file(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        let path = path.as_ref();

        self.set_filename(path)?;

        let lines = if fs::exists(path)? {
            fs::read_to_string(path)?
                .lines()
                .map(str::to_string)
                .collect()
        } else {
            Vec::new()
        };

        self.load_buffer(lines);

        Ok(())

    }

    /// Returns [`None`] if the buffer is not backed by any file.
    pub fn save_to_loaded_file(&self) -> Option<io::Result<()>> {
        Some(self.save_to_file(self.filename.as_ref()?))
    }

    pub fn save_to_file(&self, filename: impl AsRef<Path>) -> io::Result<()> {

        if self.getlines().is_empty() {
            return Ok(());
        }

        let buf = self.lines.join("\n");

        fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(filename)?
            .write_all(buf.as_bytes())?;

        Ok(())
    }

    //
    // Internal helpers
    //

    /// moves the cursor to a valid position if it is out-of-bounds.
    /// used after most operations that modify the cursor or data.
    fn check_cursor(&mut self) {

        // line cursor has to be verified before char cursor,
        // as it may hold an invalid value, such that indexing into
        // the lines vec may panic
        let max_line = self.lines.len() as isize - 1;
        self.cursor.y = self.cursor.y
            .clamp(0, max_line);

        let len = self.getline().len() as isize;

        if self.getline().is_empty() {
            self.append = true;
        } else if self.cursor.x != len {
            self.append = false;
        }

        let max_char = if self.append { len } else { len - 1 };
        self.cursor.x = self.cursor.x
            .clamp(0, max_char);

    }

    //
    // Getter API
    //

    #[must_use]
    pub fn filename(&self) -> Option<&Path> {
        self.filename.as_deref()
    }

    #[must_use]
    pub fn cursor(&self) -> Cursor {
        self.cursor
    }

    #[must_use]
    pub fn getlines(&self) -> &[String] {
        &self.lines
    }

    #[must_use]
    pub fn getline(&self) -> &str {
        &self.lines[self.cursor.y as usize]
    }

    /// Returns [`None`] if cursor is out-of-bounds (append mode)
    /// in which case the cursor is not pointing to any valid char
    #[must_use]
    pub fn getchar(&self) -> Option<char> {
        self.lines[self.cursor.y as usize]
            .chars()
            .nth(self.cursor.x as usize)
    }

    //
    // Insert API
    //

    pub fn newline_above(&mut self) {
        self.lines.insert(self.cursor.y as usize, String::new());
        // cursor will be on the newly inserted line,
        // and could therefore be out-of-bounds
        self.check_cursor();
    }

    pub fn newline_below(&mut self) {
        self.lines.insert(self.cursor.y as usize + 1, String::new());
    }

    pub fn insert_string(&mut self, str: impl AsRef<str>) {
        self.lines[self.cursor.y as usize]
            .insert_str(self.cursor.x as usize, str.as_ref());
    }

    pub fn insert_char(&mut self, c: char) {
        self.lines[self.cursor.y as usize]
            .insert(self.cursor.x as usize, c);
    }

    /// Splits the text at the cursor into two lines.
    /// Intended to be used in insert mode.
    pub fn split_newline(&mut self) {
        let line = &mut self.lines[self.cursor.y as usize];

        let str = line.split_off(self.cursor.x as usize);
        self.lines.insert(self.cursor.y as usize + 1, str);
        self.move_start_line();
        self.move_down();
    }

    //
    // Deletion API
    //

    pub fn clear_current_line(&mut self) {
        self.lines[self.cursor.y as usize].clear();
        self.check_cursor();
    }

    pub fn delete_line(&mut self) {

        if self.lines.len() == 1 {
            self.clear_current_line();
        } else {
            self.lines.remove(self.cursor.y as usize);
            self.check_cursor();
        }

    }

    /// Deletes the character at the cursor
    pub fn delete_char(&mut self) {
        if self.getline().is_empty() { return }

        self.lines[self.cursor.y as usize]
            .remove(self.cursor.x as usize);
        self.check_cursor();
    }

    //
    // Movement API
    //

    pub fn move_down_many(&mut self, count: isize) {
        self.cursor.y += count;
        self.check_cursor();
    }

    pub fn move_up_many(&mut self, count: isize) {
        self.cursor.y -= count;
        self.check_cursor();
    }

    pub fn move_down(&mut self) {
        self.cursor.y += 1;
        self.check_cursor();
    }

    pub fn move_up(&mut self) {
        self.cursor.y -= 1;
        self.check_cursor();
    }

    pub fn move_right(&mut self) {
        self.cursor.x += 1;
        self.check_cursor();
    }

    pub fn move_left(&mut self) {
        self.cursor.x -= 1;
        self.check_cursor();
    }

    pub fn move_top(&mut self) {
        self.cursor.y = 0;
        self.check_cursor();
    }

    pub fn move_bottom(&mut self) {
        self.cursor.y = self.lines.len() as isize - 1;
        self.check_cursor();
    }

    pub fn move_start_line(&mut self) {
        self.cursor.x = 0;
        // disable append mode if enabled
        self.check_cursor();
    }

    /// move to the end of the current line (one char after last char)
    pub fn move_append_end_line(&mut self) {
        self.append = true;
        self.move_end_line();
        self.move_right();
    }

    /// move to the last char of the current line
    pub fn move_end_line(&mut self) {
        self.cursor.x = self.getline().len() as isize - 1;
    }

    //
    // Miscellaneous
    //

    /// Searches the buffer for all occurances of the given substring.
    /// Returns a Vec of indices to the start of the found substrings.
    pub fn search(&self) -> Vec<Cursor> {

        let query = &self.search_query;
        self.lines
            .iter()
            // TODO: what about multiple search results?
            .enumerate()
            // TODO: refactor
            .filter_map(|(idx, line)| line.find(query).map(|q| (idx as isize, q as isize).into()))
            .collect()
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffers_count() {
        let mut buffers = Buffers::default();
        buffers.add();
        buffers.add();
        buffers.add();
        assert!(buffers.get(0).is_some());
        assert!(buffers.get(1).is_some());
        assert!(buffers.get(2).is_some());
        assert!(buffers.get(3).is_none());
        assert_eq!(buffers.count(), 3);
    }

    #[test]
    fn test_buffer_append() {
        let mut buf = Buffer::new();

        assert!(buf.append);
        buf.insert_string("foo");

        buf.move_start_line();
        assert!(!buf.append);

        buf.move_append_end_line();
        assert!(buf.append);
    }

    #[test]
    fn test_buffer_move_left_align() {
        let mut buf = Buffer::new();
        buf.insert_string("foo");
        buf.move_left();
        assert_eq!(buf.cursor.x, 0);
    }

    #[test]
    fn test_buffer_move_right_align() {
        let mut buf = Buffer::new();
        buf.insert_string("foo");
        buf.move_right();
        buf.move_right();
        buf.move_right();
        assert_eq!(buf.cursor.x, 2);
    }

    #[test]
    fn test_buffer_move_up_align() {
        let mut buf = Buffer::new();
        buf.insert_string("foo");
        buf.newline_below();
        buf.move_down();
        buf.insert_string("foobarbaz");
        assert_eq!(buf.lines, vec![ "foo", "foobarbaz" ]);
        buf.move_end_line();

        buf.move_up();
        assert_eq!(buf.cursor.x, 2);
    }

    #[test]
    fn test_buffer_move_down_align() {
        let mut buf = Buffer::new();
        buf.insert_string("foobarbaz");
        buf.newline_below();
        buf.move_down();
        buf.insert_string("foo");
        assert_eq!(buf.lines, vec![ "foobarbaz", "foo" ]);
        buf.move_up();
        buf.move_end_line();

        buf.move_down();
        assert_eq!(buf.cursor.x, 2);
    }

    #[test]
    fn test_buffer_split() {
        let mut buf = Buffer::new();
        buf.insert_char('h');
        buf.move_right();
        buf.insert_char('e');
        buf.move_right();
        buf.insert_char('l');
        buf.move_right();
        buf.insert_char('l');
        buf.move_right();
        buf.insert_char('o');
        buf.move_left();
        buf.split_newline();
        assert_eq!(buf.lines, vec![ "hel", "lo" ]);
    }

    #[test]
    fn test_buffer_insert_chars() {
        let mut buf = Buffer::new();
        buf.insert_char('a');
        buf.insert_char('b');
        buf.insert_char('c');
        buf.newline_below();
        buf.move_down();
        buf.insert_char('x');
        buf.insert_char('y');
        buf.insert_char('z');
        assert_eq!(buf.lines, vec![ "cba", "zyx" ]);
    }

}
