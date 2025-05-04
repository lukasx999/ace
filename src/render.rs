use macroquad::prelude::*;

use crate::edit::{Editor, Statusline};

const PADDING: f32 = 30.;
const COLOR_WIDGET_AREA_SEL: Color = Color::from_rgba(70, 74, 79, 255);
const COLOR_WIDGET_AREA:     Color = Color::from_rgba(57, 60, 64, 255);

mod canvas;
mod buffer;
mod statusline;
mod window;

use canvas::CanvasRenderer;
use statusline::StatuslineRenderer;



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
            padding: PADDING,
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
