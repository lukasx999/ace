use macroquad::prelude::*;
use statusline::StatuslineRenderer;

use crate::{edit::{window::Window, Editor, Statusline}, wrap::draw_rectangle_rect, COLOR_WIDGET_AREA, COLOR_WIDGET_AREA_SEL};

use macroquad::prelude::*;

mod buffer;
mod statusline;
mod window;
use window::WindowRenderer;
use buffer::BufferRenderer;


    // +---------------------------------------------------------------------+
    // |                                canvas                               |
    // |  +----------------------+                 +----------------------+  |
    // |  |       window         |                 |        window        |  |
    // |  |  +----------------+  |                 |  +----------------+  |  |
    // |  |  |    buffer      |  |                 |  |     buffer     |  |  |
    // |  |  |                |  |                 |  |                |  |  |
    // |  |  |                |  |                 |  |                |  |  |
    // |  |  |                |  |                 |  |                |  |  |
    // |  |  |                |  |                 |  |                |  |  |
    // |  |  |                |  |                 |  |                |  |  |
    // |  |  +----------------+  |                 |  +----------------+  |  |
    // |  |                      |                 |                      |  |
    // |  +----------------------+                 +----------------------+  |
    // |                                                                     |
    // +---------------------------------------------------------------------+
    // |                                statusbar                            |
    // +---------------------------------------------------------------------+





#[derive(Debug, Clone)]
pub struct CanvasRenderer {
    win: WindowRenderer,
}

impl CanvasRenderer {

    pub async fn new() -> Result<Self, macroquad::Error> {
        Ok(Self {
            win: WindowRenderer::new().await?,
        })
    }

    pub fn render(&mut self, bounds: Rect, ed: &Editor) {

        let len = ed.windows().count();
        let width = bounds.w / len as f32;

        // TODO: refactor
        let windows = ed.windows().windows.clone();
        for (idx, (winid, win)) in windows.iter().enumerate() {

            let win_bounds = Rect {
                w: width,
                x: idx as f32 * width + bounds.x,
                ..bounds
            };

            let current = ed.windows().winid().unwrap();
            let active = *winid == current;
            self.win.render(win_bounds, active, ed, win);
        }
    }


}

#[derive(Debug, Clone)]
pub struct Renderer {
    canvas: CanvasRenderer,
    statusline: StatuslineRenderer,
    padding: f32,
}

impl Renderer {

    pub async fn new() -> Result<Self, macroquad::Error> {
        Ok(Self {
            canvas: CanvasRenderer::new().await?,
            statusline: StatuslineRenderer::new().await?,
            padding: 30.,
        })
    }

    #[must_use]
    pub fn padding(&self) -> f32 {
        self.padding
    }

    pub fn set_padding(&mut self, padding: f32) {
        self.padding = padding;
    }

    pub fn render(&mut self, bounds: Rect, ed: &Editor, statusline: &Statusline) {

        let status_height = self.statusline.fontsize() as f32;

        let bounds_statusline = Rect::new(
            bounds.x + self.padding,
            bounds.y + bounds.h - status_height - self.padding,
            bounds.w - self.padding * 2.,
            status_height,
        );

        let bounds_canvas = Rect::new(
            bounds.x + self.padding,
            bounds.y + self.padding,
            bounds.w - self.padding * 2.,
            bounds.h - bounds_statusline.h - self.padding * 2.,
        );

        self.statusline.render(bounds_statusline, statusline);
        self.canvas.render(bounds_canvas, ed);
    }

}
