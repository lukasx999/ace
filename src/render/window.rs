use macroquad::prelude::*;

use crate::{
    edit::{window::Window, Editor, Statusline},
    wrap::draw_rectangle_rect,
    COLOR_WIDGET_AREA,
    COLOR_WIDGET_AREA_SEL,
};
use super::BufferRenderer;


#[derive(Debug, Clone)]
pub struct WindowRenderer {
    buf: BufferRenderer,
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

        if let Some(id) = win.bufid {

            let buf = ed
                .buffers()
                .get(id)
                .unwrap();
            self.buf.render(bounds, buf, ed.mode(), active);

        }
        // TODO: some sort of indicator for empty windows
    }


}
