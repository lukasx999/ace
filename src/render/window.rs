use macroquad::prelude::*;

use crate::wrap::draw_rectangle_rect;
use crate::edit::Editor;
use crate::edit::window::Window;

use super::{buffer::BufferRenderer, COLOR_WIDGET_AREA, COLOR_WIDGET_AREA_SEL};



#[derive(Debug, Clone)]
pub struct WindowRenderer {
    pub buf: BufferRenderer,
}

impl WindowRenderer {
    pub async fn new() -> Result<Self, macroquad::Error> {
        Ok(Self {
            buf: BufferRenderer::new().await?,
        })
    }

    pub fn render(&mut self, bounds: Rect, active: bool, ed: &Editor, win: &Window) {

        draw_rectangle_rect(bounds, if active {
            COLOR_WIDGET_AREA_SEL
        } else {
            COLOR_WIDGET_AREA
        });

        if let Some(id) = win.buf {

            let buf = ed
                .buffers()
                .get(id)
                .unwrap();
            self.buf.render(bounds, buf, ed.mode(), active);

        }
        // TODO: some sort of indicator for empty windows
    }


}
