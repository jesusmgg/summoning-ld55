use std::mem;

use macroquad::{
    camera::{set_camera, set_default_camera, Camera2D},
    input::mouse_position,
    math::{f32, Rect, Vec2},
};

const MAX_CAMERA_COUNT: usize = 8;

pub struct CameraMgr {
    camera: Vec<Camera2D>,

    active_camera_i: Option<usize>,
}

impl CameraMgr {
    pub fn new() -> Self {
        let camera = Vec::with_capacity(MAX_CAMERA_COUNT);
        let active_camera_i = None;

        Self {
            camera,
            active_camera_i,
        }
    }

    pub fn add_default(&mut self) -> usize {
        let rect = Rect::new(0.0, 0.0, 960.0, 540.0);
        let target = f32::vec2(rect.x + rect.w / 2., rect.y + rect.h / 2.);
        let zoom = f32::vec2(1. / rect.w * 2., 1. / rect.h * 2.);

        let camera = Camera2D {
            target,
            zoom,
            offset: f32::vec2(0., 0.),
            rotation: 0.,

            render_target: None,
            viewport: None,
        };

        self.camera.push(camera);

        let index = self.len() - 1;

        index
    }

    pub fn len(&self) -> usize {
        self.camera.len()
    }

    pub fn init(&mut self) {
        let index = self.add_default();
        self.set_active_camera(index);
    }

    pub fn set_active_camera(&mut self, index: usize) {
        let camera = &self.camera[index];
        set_camera(camera);
        self.active_camera_i = Some(index);
    }

    pub fn active_camera(&self) -> Option<&Camera2D> {
        match self.active_camera_i {
            Some(i) => Some(&self.camera[i]),
            None => None,
        }
    }

    /// Get mouse world position using current camera
    pub fn get_mouse_world_position(&self) -> f32::Vec2 {
        let mut mouse_screen_pos: f32::Vec2 = unsafe { mem::MaybeUninit::zeroed().assume_init() };
        (mouse_screen_pos.x, mouse_screen_pos.y) = mouse_position();

        let camera = self.active_camera().unwrap();

        camera.screen_to_world(mouse_screen_pos)
    }
}
