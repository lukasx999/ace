const KEY_REPEAT_DELAY: f64 = 5.0;

use std::collections::HashMap;
use std::time::Duration;

use macroquad::prelude::*;

// should be called in the main loop
// sleeps in the current iteration until the desired framerate is achieved
pub fn sleep_framerate(framerate: f32) {

    let minft = 1. / framerate;
    let ft = get_frame_time();

    if ft < minft {
        let sleep = minft - ft;
        std::thread::sleep(Duration::from_secs_f32(sleep));
    }

}

/// Truncates a string slice to the given width
pub fn clamp_slice<'a>(mut slice: &'a str, max_width: f32, params: TextParams) -> &'a str {
    while measure_text(
        slice,
        params.font,
        params.font_size,
        params.font_scale,
    ).width > max_width {
        slice = &slice[..slice.len()-1];
    }
    slice
}

pub fn measure_char(
    c:          char,
    font:       Option<&Font>,
    font_size:  u16,
    font_scale: f32
) -> TextDimensions {
    measure_text(c.to_string().as_str(), font, font_size, font_scale)
}

pub fn draw_rectangle_rect(rect: Rect, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color)
}

pub fn draw_text_bounded(
    text:      impl AsRef<str>,
    x:         f32,
    y:         f32,
    params:    TextParams,
    max_width: f32,
) -> TextDimensions {

    // get slice of text that is small enough to render
    let mut text_slice = text.as_ref();

    while measure_text(text_slice, params.font, params.font_size, params.font_scale).width > max_width {
        text_slice = &text_slice[..text_slice.len()-1];
    }

    // adding font size to y, such that the text origin is at
    // top left, rather than bottom left, which is more intuitive (imo)
    draw_text_ex(text_slice, x, y + params.font_size as f32, params)

}

pub fn get_last_key_down() -> Option<KeyCode> {
    get_keys_down().into_iter().next()
}

#[allow(static_mut_refs)]
pub fn is_key_repeated(key_code: KeyCode) -> bool {

    static mut TIME_MAP: Option<HashMap<KeyCode, f64>> = None;

    unsafe {

        if TIME_MAP.is_none() {
            TIME_MAP = Some(HashMap::new());
        }

        let time = TIME_MAP
            .as_mut()
            .unwrap();

        time.entry(key_code).or_insert(0.);

        let time_pressed = time
            .get_mut(&key_code)
            .unwrap();

        if is_key_down(key_code) {
            *time_pressed += 1.;
        } else {
            *time_pressed = 0.;
        }

        *time_pressed >= KEY_REPEAT_DELAY

    }
}

pub fn is_key_active(key_code: KeyCode) -> bool {
    is_key_pressed(key_code) || is_key_repeated(key_code)
}

pub fn is_shift_down() -> bool {
    is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift)
}

pub fn is_ctrl_down() -> bool {
    is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)
}

pub fn is_meta_down() -> bool {
    is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt)
}

pub fn is_super_down() -> bool {
    is_key_down(KeyCode::LeftSuper) || is_key_down(KeyCode::RightSuper)
}
