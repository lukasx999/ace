use macroquad::prelude::*;

use crate::edit::Editor;
use super::window::WindowRenderer;



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
