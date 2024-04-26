use macroquad::{color, text::draw_text, time};

use super::camera::camera::CameraMgr;

pub struct DiagnosticsMgr {}

impl DiagnosticsMgr {
    pub fn new() -> Self {
        Self {}
    }

    pub fn init(&self) {}

    pub fn render(&self, camera_mgr: &CameraMgr) {
        camera_mgr.push_active_camera();

        let font_size = 32.0;

        let frame_time_text = format!("Frame time: {:.2}ms", time::get_frame_time() * 1000.0);
        let fps_text = format!("FPS       : {:.2}", time::get_fps());
        draw_text(fps_text.as_str(), 1.0, 19.0, font_size, color::BLACK);
        draw_text(fps_text.as_str(), 0.0, 18.0, font_size, color::WHITE);

        draw_text(frame_time_text.as_str(), 1.0, 39.0, font_size, color::BLACK);
        draw_text(frame_time_text.as_str(), 0.0, 38.0, font_size, color::WHITE);

        draw_text("Press <q> to quit", 1.0, 59.0, font_size, color::BLACK);
        draw_text("Press <q> to quit", 0.0, 58.0, font_size, color::LIGHTGRAY);

        camera_mgr.pop_active_camera();
    }
}
