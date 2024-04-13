use macroquad::{color, text::draw_text, time, window::screen_height};

pub struct DiagnosticsMgr {}

impl DiagnosticsMgr {
    pub fn new() -> Self {
        Self {}
    }

    pub fn init(&self) {}

    pub fn render(&self) {
        let frame_time_text = format!("Frame time: {:.2}ms", time::get_frame_time() * 1000.0);
        let fps_text = format!("FPS       : {:.2}", time::get_fps());
        draw_text(
            fps_text.as_str(),
            0.0,
            screen_height() - 56.0,
            32.0,
            color::WHITE,
        );
        draw_text(
            frame_time_text.as_str(),
            0.0,
            screen_height() - 32.0,
            32.0,
            color::WHITE,
        );
        draw_text(
            "Press <q> to quit",
            0.0,
            screen_height() - 8.0,
            32.0,
            color::LIGHTGRAY,
        );
    }
}
