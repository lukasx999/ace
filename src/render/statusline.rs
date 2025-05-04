use macroquad::prelude::*;
use crate::edit::Statusline;
use crate::wrap::{draw_rectangle_rect, draw_text_bounded};
use super::COLOR_WIDGET_AREA;

const COLOR_STATUSLINE: Color = Color::from_rgba(158, 189, 219, 255);
const FONTPATH:         &str  = "/usr/share/fonts/TTF/Roboto-Regular.ttf";
const FONTSIZE:         u16   = 30;

#[derive(Debug, Clone)]
pub struct StatuslineRenderer {
    font: Font,
    fontsize: u16,
}

impl StatuslineRenderer {

    pub async fn new() -> Result<Self, macroquad::Error> {
        Ok(Self {
            font: load_ttf_font(FONTPATH).await?,
            fontsize: FONTSIZE,
        })
    }

    pub fn fontsize(&self) -> u16 {
        self.fontsize
    }

    fn textwidth(&self, text: impl AsRef<str>) -> f32 {
        measure_text(text.as_ref(), Some(&self.font), self.fontsize, 1.).width
    }

    pub fn render(&mut self, bounds: Rect, statusline: &Statusline) {

        draw_rectangle_rect(bounds, COLOR_WIDGET_AREA);

        let params = TextParams {
            font:      Some(&self.font),
            font_size: self.fontsize,
            color:     COLOR_STATUSLINE,
            ..Default::default()
        };

        draw_text_bounded(
            &statusline.left,
            bounds.x,
            bounds.y,
            params.clone(),
            bounds.w
        );

        let width = self.textwidth(&statusline.center);

        draw_text_bounded(
            &statusline.center,
            bounds.x + bounds.w / 2. - width / 2.,
            bounds.y,
            params.clone(),
            bounds.w
        );

        let width = self.textwidth(&statusline.right);

        draw_text_bounded(
            &statusline.right,
            bounds.x + bounds.w - width,
            bounds.y,
            params,
            bounds.w
        );

    }
}
