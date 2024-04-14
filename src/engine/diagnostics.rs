use macroquad::{color, text::draw_text, time};

pub struct DiagnosticsMgr {}

impl DiagnosticsMgr {
    pub fn new() -> Self {
        Self {}
    }

    pub fn init(&self) {}

    // TODO: scale with camera
    pub fn render(&self) {
        let frame_time_text = format!("Frame time: {:.2}ms", time::get_frame_time() * 1000.0);
        let fps_text = format!("FPS       : {:.2}", time::get_fps());
        draw_text(fps_text.as_str(), 1.0, 11.0, 16.0, color::BLACK);
        draw_text(fps_text.as_str(), 0.0, 10.0, 16.0, color::WHITE);

        draw_text(frame_time_text.as_str(), 1.0, 25.0, 16.0, color::BLACK);
        draw_text(frame_time_text.as_str(), 0.0, 24.0, 16.0, color::WHITE);

        draw_text("Press <q> to quit", 1.0, 39.0, 16.0, color::BLACK);
        draw_text("Press <q> to quit", 0.0, 38.0, 16.0, color::LIGHTGRAY);
    }
}
