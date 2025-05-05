use macroquad::prelude::*;
use crate::edit::Mode;
use crate::edit::buffer::{Buffer, Cursor};
use crate::wrap::{clamp_slice, draw_text_bounded, measure_char};

const CURSOR_SIZE:      f32   = 3.;
const COLOR_CURSORLINE: Color = Color::from_rgba(71, 76, 82, 255);
const COLOR_CURSOR:     Color = Color::from_rgba(186, 194, 204, 255);
const COLOR_TEXT:       Color = Color::from_rgba(255, 255, 255, 255);
const FONTPATH:         &str  = "/usr/share/fonts/TTF/JetBrainsMonoNerdFont-Regular.ttf";
const FONTSIZE:         u16   = 30;



#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorShape {
    #[default] Block,
    Beam,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineNumberMode {
    #[default] Relative,
    Absolute,
}

/// Information the renderer holds while rendering a single frame
#[derive(Debug, Clone)]
struct BufferRenderArgs<'a> {
    buf: &'a Buffer,
    mode: Mode,
    linecount_vis: usize,
    charcount_vis: usize,
    bounds_buf: Rect,
    bounds_linenumbers: Rect,
    virt: Cursor,
    params: TextParams<'a>,
}

#[derive(Debug, Clone)]
pub struct BufferRenderer {
    /// Settings
    mode: LineNumberMode,
    font: Font,
    fontsize: u16,

    /// State
    buf_offset: Cursor,
}

impl BufferRenderer {

    pub async fn new() -> Result<Self, macroquad::Error> {
        Ok(Self {
            mode: LineNumberMode::default(),
            font: load_ttf_font(FONTPATH).await?,
            fontsize: FONTSIZE,
            buf_offset: Cursor::default(),
        })
    }

    pub fn set_mode(&mut self, mode: LineNumberMode) {
        self.mode = mode;
    }

    pub fn set_fontsize(&mut self, fontsize: u16) {
        self.fontsize = fontsize;
    }

    #[must_use]
    pub fn fontsize(&self) -> u16 {
        self.fontsize
    }

    fn empty_column_width(&self) -> f32 {
        measure_char('X', Some(&self.font), self.fontsize, 1.).width
    }

    fn textwidth(&self, text: impl AsRef<str>) -> f32 {
        measure_text(text.as_ref(), Some(&self.font), self.fontsize, 1.).width
    }

    fn draw_line_cursor(&self, bounds: Rect, fontsize: f32, virt_y: isize) {

        draw_rectangle(
            bounds.x,
            bounds.y + virt_y as f32 * fontsize,
            bounds.w,
            fontsize,
            COLOR_CURSORLINE,
        );

    }

    fn draw_cursor(&self, args: &BufferRenderArgs) {

        let fontsize = self.fontsize as f32;

        self.draw_line_cursor(args.bounds_buf, fontsize, args.virt.y);

        // width of all chars leading up to cursor
        let line = &args.buf.lines[args.buf.cur.y as usize];
        let widthsum = self.textwidth(&line[..args.virt.x as usize]);

        // width of current char
        let c = args.buf.getchar().unwrap_or(' ');

        let cursor = match args.mode {
            Mode::Normal => self.textwidth(c.to_string()),
            Mode::Insert => CURSOR_SIZE
        };

        // char cursor
        draw_rectangle(
            args.bounds_buf.x + widthsum,
            args.bounds_buf.y + args.virt.y as f32 * fontsize,
            cursor,
            fontsize,
            COLOR_CURSOR,
        );
    }

    fn draw_linenumbers(&mut self, args: &BufferRenderArgs, i: usize) {

        let bounds = args.bounds_linenumbers;

        let abs = i + self.buf_offset.y as usize;
        let is_current = args.buf.cur.y as usize == abs;

        let linenum = match self.mode {
            LineNumberMode::Absolute => abs + 1,
            LineNumberMode::Relative =>
            if is_current {
                abs + 1
            } else {
                abs.abs_diff(args.buf.cur.y as usize)
            },
        };

        let text = linenum.to_string();

        draw_text_bounded(
            &text,
            bounds.x + bounds.w - self.textwidth(&text) - self.empty_column_width(),
            bounds.y + i as f32 * self.fontsize as f32,
            args.params.clone(),
            bounds.w
        );

    }


    fn draw_lines(&mut self, args: &BufferRenderArgs) {

        let y = self.buf_offset.y as usize;
        // edge-case: deleting lines when scrolled to the end of buffer,
        // hence clamping to document length
        let len = (y + args.linecount_vis)
            .min(args.buf.getlines().len());

        let lines = &args.buf.lines[y..len];

        for (i, line) in lines.iter().enumerate() {

            let line = &line[(self.buf_offset.x as usize)
                .min(line.len())..];

            // TODO: make char under cursor black
            draw_text_bounded(
                line,
                args.bounds_buf.x,
                args.bounds_buf.y + i as f32 * self.fontsize as f32,
                args.params.clone(),
                args.bounds_buf.w
            );

            self.draw_linenumbers(args, i);

        }
    }


    fn check_cursor_x(&mut self, args: &BufferRenderArgs) {

        let diffx = args.virt.x - args.charcount_vis as isize;

        if diffx >= 0 {
            self.buf_offset.x += diffx + 1;
        }

        if args.virt.x < 0 {
            self.buf_offset.x += args.virt.x;
        }
    }

    fn check_cursor_y(&mut self, args: &BufferRenderArgs) {

        // if cursor is out-of-bounds, move it back by how much it moved out-of-bounds
        let diff = args.virt.y - args.linecount_vis as isize;
        // always jump by at least one,
        // as otherwise offset will not change when diff is 0
        if diff >= 0 {
            self.buf_offset.y += diff + 1;
        }

        if args.virt.y < 0 {
            self.buf_offset.y += args.virt.y;
        }
    }

    pub fn render(&mut self, bounds: Rect, buf: &Buffer, mode: Mode, active: bool) {

        let font = self.font.clone();
        let params = TextParams {
            font:      Some(&font),
            font_size: self.fontsize,
            color:     COLOR_TEXT,
            ..Default::default()
        };

        let column_len = self.textwidth(buf.getlines().len().to_string()) + self.empty_column_width();

        let bounds_linenumbers = Rect { w: column_len, ..bounds };

        let bounds_buf = Rect {
            x: bounds.x + column_len,
            w: bounds.w - column_len,
            ..bounds
        };



        // the amount of lines that can fit onto the screen
        let linecount_vis = ((bounds_buf.h / self.fontsize as f32) as usize)
            .min(buf.lines.len());

        // the amount of chars that can fit onto the current line
        // TODO: refactor clamp slice
        let charcount_vis = clamp_slice(buf.getline(), bounds_buf.w, params.clone()).len();

        // absolute cursor position mapped to the
        // actual visible bounds of the buffer (virtual cursor)
        let virt = buf.cur - self.buf_offset;

        let mut args = BufferRenderArgs {
            buf,
            mode,
            linecount_vis,
            charcount_vis,
            bounds_buf,
            bounds_linenumbers,
            virt,
            params,
        };

        self.check_cursor_y(&args);
        self.check_cursor_x(&args);

        // recalculate cursor, if offset changed, otherwise there will be
        // a cursor jumping effect at the top and bottom
        args.virt = buf.cur - self.buf_offset;


        if active {
            self.draw_cursor(&args);
        }

        self.draw_lines(&args);

    }
}
