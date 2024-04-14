use macroquad::{
    color,
    math::{f32, Rect},
};

use crate::engine::sprite::SpriteMgr;

const MAX_COLLIDER_COUNT: usize = 256;
/// Determines the maximum amount of collisions that will be returned from a collision test.
pub const MAX_COLLISION_COUNT: usize = 16;

/// Manages bounding boxes and collisions.
/// TODO: allow passing in a list of colliders to test against.
/// TODO: check `is_active` field in collision tests.
pub struct ColliderMgr {
    pub bbox: Vec<Rect>,

    /// Whether to render the bounding box or not.
    pub render_bbox: Vec<bool>,
    is_active: Vec<bool>,
}

impl ColliderMgr {
    pub fn new() -> Self {
        Self {
            bbox: Vec::with_capacity(MAX_COLLIDER_COUNT),

            render_bbox: Vec::with_capacity(MAX_COLLIDER_COUNT),
            is_active: Vec::with_capacity(MAX_COLLIDER_COUNT),
        }
    }

    pub fn len(&self) -> usize {
        self.bbox.len()
    }

    pub fn is_active(&self, index: usize) -> bool {
        self.is_active[index]
    }

    pub fn set_active(&mut self, index: usize, is_active: bool) {
        self.is_active[index] = is_active;
    }

    pub fn add(&mut self, bbox: Rect) -> usize {
        self.bbox.push(bbox);
        self.render_bbox.push(false);
        self.is_active.push(true);

        let index = self.len() - 1;

        index
    }

    /// Adds a new collider with a sprite's position and size by default, or a predefined `bbox` if
    /// passed as argument.
    pub fn add_from_sprite(
        &mut self,
        sprite_i: usize,
        bbox: Option<Rect>,
        sprite_mgr: &mut SpriteMgr,
    ) -> usize {
        let bbox = bbox.unwrap_or(Self::create_rect_for_sprite(sprite_i, sprite_mgr));

        self.add(bbox)
    }

    /// Creates a new `Rect` with the sprite's position and size that can be used as a bounding box.
    pub fn create_rect_for_sprite(sprite_i: usize, sprite_mgr: &mut SpriteMgr) -> Rect {
        let position = &sprite_mgr.position[sprite_i];
        let size = sprite_mgr.scaled_size(sprite_i);

        Rect::new(position.x, position.y, size.x, size.y)
    }

    /// Tests a `point` for intersection with other colliders.
    ///
    /// Returns the collision count.
    pub fn intersect_point(
        &self,
        index: usize,
        point: &f32::Vec2,
        hit_buffer: &mut Vec<Hit>,
    ) -> usize {
        let mut collision_count: usize = 0;

        for i in 0..self.len() {
            if i == index || !self.is_active(i) {
                continue;
            }

            let mut hit = &mut hit_buffer[collision_count];

            let bbox = self.bbox[i];
            let center = bbox.center();

            if !Self::intersect_point_single(&point, &bbox, &center, i, &mut hit) {
                continue;
            }

            collision_count += 1;

            if collision_count > MAX_COLLISION_COUNT {
                break;
            }
        }

        collision_count
    }

    pub fn intersect_point_single(
        point: &f32::Vec2,
        test_bbox: &Rect,
        test_center: &f32::Vec2,
        test_collider_i: usize,
        hit: &mut Hit,
    ) -> bool {
        let dx = point.x - test_center.x;
        let px = (test_bbox.w / 2.0) - f32::abs(dx);
        if px <= 0.0 {
            hit.is_colliding = false;
            return false;
        }

        let dy = point.y - test_center.y;
        let py = (test_bbox.h / 2.0) - f32::abs(dy);
        if py <= 0.0 {
            hit.is_colliding = false;
            return false;
        }

        hit.is_colliding = true;
        hit.collider_i = test_collider_i;
        if px < py {
            let sx = f32::signum(dx);
            hit.delta.x = px * sx;
            hit.delta.y = 0.0;
            hit.normal.x = sx;
            hit.normal.y = 0.0;
            hit.position.x = test_center.x + (test_bbox.w / 2.0) * sx;
            hit.position.y = test_center.y;
        } else {
            let sy = f32::signum(dy);
            hit.delta.x = 0.0;
            hit.delta.y = py * sy;
            hit.normal.x = 0.0;
            hit.normal.y = sy;
            hit.position.x = test_center.x;
            hit.position.y = test_center.y + (test_bbox.h / 2.0) * sy;
        }

        true
    }

    /// AABB intersection test.
    ///
    /// Checks for overlaps between bounding boxes. Gives the axis of least overlap as the contact
    /// point in the `hit.delta` field. This delta can be used to push the colliding box out of the
    /// nearest edge.
    ///
    /// Returns the collision count.
    pub fn intersect_bbox(
        &self,
        index: usize,
        translation: &f32::Vec2,
        hit_buffer: &mut Vec<Hit>,
        ignore_list: Option<&Vec<usize>>,
    ) -> usize {
        let mut collision_count: usize = 0;

        let mut self_bbox = self.bbox[index];
        self_bbox.x += translation.x;
        self_bbox.y += translation.y;
        let self_center = self_bbox.center();

        for i in 0..self.len() {
            if i == index || !self.is_active(i) {
                continue;
            }

            match ignore_list {
                Some(list) => {
                    if list.contains(&i) {
                        continue;
                    }
                }
                None => {}
            }

            let bbox = self.bbox[i];
            let center = bbox.center();

            let mut hit = &mut hit_buffer[collision_count];

            if !Self::intersect_bbox_single(&self_bbox, &self_center, &bbox, &center, i, &mut hit) {
                continue;
            }

            collision_count += 1;

            if collision_count > MAX_COLLISION_COUNT {
                break;
            }
        }

        collision_count
    }

    fn intersect_bbox_single(
        self_bbox: &Rect,
        self_center: &f32::Vec2,
        test_bbox: &Rect,
        test_center: &f32::Vec2,
        test_collider_i: usize,
        hit: &mut Hit,
    ) -> bool {
        let dx = test_center.x - self_center.x;
        let px = (test_bbox.w / 2.0) + (self_bbox.w / 2.0) - f32::abs(dx);
        if px <= 0.0 {
            hit.is_colliding = false;
            return false;
        }
        let dy = test_center.y - self_center.y;
        let py = (test_bbox.h / 2.0) + (self_bbox.h / 2.0) - f32::abs(dy);
        if py <= 0.0 {
            hit.is_colliding = false;
            return false;
        }

        hit.is_colliding = true;
        hit.collider_i = test_collider_i;
        if px < py {
            let sx = f32::signum(dx);
            hit.delta.x = px * sx;
            hit.delta.y = 0.0;
            hit.normal.x = sx;
            hit.normal.y = 0.0;
            hit.position.x = self_center.x + (self_bbox.x / 2.0) * sx;
            hit.position.y = test_center.y;
        } else {
            let sy = f32::signum(dy);
            hit.delta.x = 0.0;
            hit.delta.y = py * sy;
            hit.normal.x = 0.0;
            hit.normal.y = sy;
            hit.position.x = test_center.x;
            hit.position.y = self_center.y + (self_bbox.y / 2.0) * sy;
        }

        true
    }

    pub fn create_hit_buffer(size: usize) -> Vec<Hit> {
        let mut buffer: Vec<Hit> = Vec::with_capacity(size);

        for _i in 0..size {
            buffer.push(Hit {
                is_colliding: false,
                collider_i: usize::MAX,
                position: f32::Vec2::ZERO,
                delta: f32::Vec2::ZERO,
                normal: f32::Vec2::ZERO,
            })
        }

        buffer
    }

    pub fn set_position(&mut self, index: usize, x: f32, y: f32) {
        self.bbox[index].x = x;
        self.bbox[index].y = y;
    }

    pub fn render(&self) {
        for i in 0..self.len() {
            if !self.render_bbox[i] || !self.is_active(i) {
                continue;
            }

            let bbox = self.bbox[i];
            macroquad::shapes::draw_rectangle_lines(
                bbox.x,
                bbox.y,
                bbox.w,
                bbox.h,
                4.0,
                color::WHITE,
            );
        }
    }
}

#[derive(Debug)]
pub struct Hit {
    /// If `false`, assume every other field contains invalid data.
    pub is_colliding: bool,
    pub collider_i: usize,
    /// Contact point of the two objects.
    pub position: f32::Vec2,
    /// Overlap between the two objects.
    /// This vector can be added to the colliding object's position to move it back to a
    /// non-colliding state.
    pub delta: f32::Vec2,
    /// Surface normal at the point of contact.
    pub normal: f32::Vec2,
}
