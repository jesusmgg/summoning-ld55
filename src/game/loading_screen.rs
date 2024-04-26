use macroquad::{
    color,
    text::{draw_text, get_text_center},
    window::{clear_background, screen_height, screen_width},
};

use crate::engine::{camera::camera::CameraMgr, scene::SceneMgr};

pub struct LoadingScreen;

/// Shows a loading screen while the SceneManager is loading or unloading a scene.
impl LoadingScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, scene_mgr: &SceneMgr, camera_mgr: &CameraMgr) {
        if scene_mgr.has_pending_spawn() || scene_mgr.has_pending_despawn() {
            clear_background(color::RED);

            camera_mgr.push_active_camera();

            let loading_text = "Loading...";
            let text_size = 48;
            let text_center = get_text_center(loading_text, Option::None, text_size, 1.0, 0.0);
            let text_x = screen_width() / 2.0 - text_center.x;
            let text_y = screen_height() / 2.0 - text_center.y;

            draw_text(
                loading_text,
                text_x.round(),
                text_y.round(),
                text_size as f32,
                color::WHITE,
            );

            camera_mgr.pop_active_camera();
        }
    }
}
