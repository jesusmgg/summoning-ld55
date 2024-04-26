use crate::engine::{
    camera::camera::CameraMgr,
    collision::collider::{self, ColliderMgr, Hit},
};

use macroquad::{
    color,
    input::{is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, MouseButton},
    math::{f32, Rect},
};

const MAX_SELECTED_COUNT: usize = 64;

/// Selects colliders with an RTS style selection box.
pub struct SelectorBox {
    collider_i: Option<usize>,

    is_dragging: bool,
    is_active: bool,
    drag_start_position: f32::Vec2,

    selected_collider_i: Vec<usize>,
    selection_hit_buffer: Vec<Hit>,
}

impl SelectorBox {
    pub fn new() -> Self {
        let collider_i = None;

        let is_dragging = false;
        let is_active = true;
        let drag_start_position = f32::Vec2::ZERO;

        let selected_collider_i = Vec::with_capacity(MAX_SELECTED_COUNT);
        let selection_hit_buffer = Vec::with_capacity(collider::MAX_COLLISION_COUNT);

        Self {
            collider_i,

            is_dragging,
            is_active,
            drag_start_position,

            selected_collider_i,
            selection_hit_buffer,
        }
    }

    pub fn init(&mut self, collider_mgr: &mut ColliderMgr) {
        let bbox = Rect::new(0.0, 0.0, 0.0, 0.0);
        self.collider_i = Some(collider_mgr.add(bbox));
        self.selection_hit_buffer = ColliderMgr::create_hit_buffer(collider::MAX_COLLISION_COUNT);
    }

    pub fn input(&mut self, camera_mgr: &CameraMgr, collider_mgr: &mut ColliderMgr) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.start_dragging(camera_mgr, collider_mgr);
        } else if is_mouse_button_down(MouseButton::Left) {
            self.update_dragging(camera_mgr, collider_mgr);
        } else if is_mouse_button_released(MouseButton::Left) {
            self.stop_dragging(collider_mgr);
        }
    }

    pub fn update(&mut self, collider_mgr: &ColliderMgr) {
        self.clear_selected_collider_i();

        let hit_count = collider_mgr.intersect_bbox(
            self.collider_i.unwrap(),
            &f32::Vec2::ZERO,
            &mut self.selection_hit_buffer,
            None,
        );

        for hit_i in 0..hit_count {
            let hit = &self.selection_hit_buffer[hit_i];
            if !hit.is_colliding {
                continue;
            }

            self.selected_collider_i.push(hit.collider_i);
        }
    }

    pub fn render(&self, collider_mgr: &ColliderMgr) {
        if !self.is_active || !self.is_dragging {
            return;
        }

        let bbox = &collider_mgr.bbox[self.collider_i.unwrap()];

        if f32::abs(bbox.w) < 1.0 || f32::abs(bbox.h) < 1.0 {
            return;
        }

        macroquad::shapes::draw_rectangle_lines(bbox.x, bbox.y, bbox.w, bbox.h, 4.0, color::YELLOW);
    }

    pub fn spawn(&mut self, collider_mgr: &mut ColliderMgr) {
        self.set_active(true, collider_mgr);
    }

    pub fn despawn(&mut self, collider_mgr: &mut ColliderMgr) {
        self.set_active(false, collider_mgr);
    }

    pub fn set_active(&mut self, is_active: bool, collider_mgr: &mut ColliderMgr) {
        self.is_active = is_active;
        collider_mgr.set_active(self.collider_i.unwrap(), is_active);
    }

    fn reset_collider(&self, collider_mgr: &mut ColliderMgr) {
        let bbox = &mut collider_mgr.bbox[self.collider_i.unwrap()];
        bbox.x = 0.0;
        bbox.y = 0.0;
        bbox.w = 0.0;
        bbox.h = 0.0;
    }

    fn start_dragging(&mut self, camera_mgr: &CameraMgr, collider_mgr: &mut ColliderMgr) {
        self.is_dragging = true;
        let mouse_pos = camera_mgr.get_mouse_world_position();
        let bbox = &mut collider_mgr.bbox[self.collider_i.unwrap()];
        self.drag_start_position = mouse_pos;
        bbox.x = self.drag_start_position.x;
        bbox.y = self.drag_start_position.y;
    }

    fn update_dragging(&mut self, camera_mgr: &CameraMgr, collider_mgr: &mut ColliderMgr) {
        let mouse_pos = camera_mgr.get_mouse_world_position();
        let bbox = &mut collider_mgr.bbox[self.collider_i.unwrap()];

        if mouse_pos.x >= self.drag_start_position.x {
            bbox.x = self.drag_start_position.x;
            bbox.w = mouse_pos.x - self.drag_start_position.x;
        } else {
            bbox.x = mouse_pos.x;
            bbox.w = self.drag_start_position.x - mouse_pos.x;
        }

        if mouse_pos.y >= self.drag_start_position.y {
            bbox.y = self.drag_start_position.y;
            bbox.h = mouse_pos.y - self.drag_start_position.y;
        } else {
            bbox.y = mouse_pos.y;
            bbox.h = self.drag_start_position.y - mouse_pos.y;
        }
    }

    fn stop_dragging(&mut self, collider_mgr: &mut ColliderMgr) {
        self.is_dragging = false;
        self.reset_collider(collider_mgr);
    }

    fn clear_selected_collider_i(&mut self) {
        if self.selected_collider_i.len() > 0 {
            self.selected_collider_i.clear();
        }
    }

    pub fn collider_i(&self) -> usize {
        self.collider_i.unwrap()
    }

    pub fn selected_collider_i(&self) -> &Vec<usize> {
        &self.selected_collider_i
    }

    pub fn is_dragging(&self) -> bool {
        self.is_active && self.is_dragging
    }
}
