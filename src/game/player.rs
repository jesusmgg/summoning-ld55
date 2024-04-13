use crate::{
    engine::collision::collider::{self, ColliderMgr, Hit},
    engine::scene::SceneMgr,
    engine::sprite::{SpriteMgr, Texture2dMgr},
};
use macroquad::input::{is_key_down, KeyCode};
use macroquad::math::f32;

pub struct Player {
    move_speed: f32,

    input_move: f32::Vec2,

    is_active: bool,

    collider_i: Option<usize>,
    sprite_i: Option<usize>,

    movement_hit_buffer: Vec<Hit>,
}

impl Player {
    pub fn new(move_speed: f32) -> Self {
        let input_move = f32::Vec2::ZERO;

        let is_active = false;

        let collider_i = None;
        let sprite_i = None;

        let movement_hit_buffer = ColliderMgr::create_hit_buffer(collider::MAX_COLLISION_COUNT);

        Self {
            move_speed,

            input_move,

            is_active,

            collider_i,
            sprite_i,

            movement_hit_buffer,
        }
    }

    pub async fn init(
        &mut self,
        sprite_mgr: &mut SpriteMgr,
        texture_mgr: &mut Texture2dMgr,
        collider_mgr: &mut ColliderMgr,
    ) {
        // Create sprite
        let sprite_i = sprite_mgr
            .add_from_file(
                "sprites/player01.png",
                f32::Vec2::ZERO,
                f32::Vec2 { x: 0.1, y: 0.1 },
                texture_mgr,
            )
            .await;
        self.sprite_i = Some(sprite_i);

        let collider_i = collider_mgr.add_from_sprite(sprite_i, None, sprite_mgr);
        self.collider_i = Some(collider_i);
        collider_mgr.render_bbox[collider_i] = true;

        self.set_active(false, sprite_mgr, collider_mgr);
    }

    /// Spawns the player at the position set in the currently active scene.
    pub fn spawn(
        &mut self,
        scene_mgr: &SceneMgr,
        sprite_mgr: &mut SpriteMgr,
        collider_mgr: &mut ColliderMgr,
    ) {
        if scene_mgr.active_scene_id == None || scene_mgr.active_objects.len() == 0 {
            return;
        }

        for i in &scene_mgr.active_objects {
            if scene_mgr.object_class[*i].as_ref().unwrap() == "Player" {
                sprite_mgr.position[self.sprite_i.unwrap()] =
                    scene_mgr.object_position[*i].unwrap();
                self.set_active(true, sprite_mgr, collider_mgr);
                break;
            }
        }
    }

    fn set_active(
        &mut self,
        is_active: bool,
        sprite_mgr: &mut SpriteMgr,
        collider_mgr: &mut ColliderMgr,
    ) {
        self.is_active = is_active;
        sprite_mgr.is_active[self.sprite_i.unwrap()] = is_active;
        collider_mgr.set_active(self.collider_i.unwrap(), is_active);
    }

    pub fn input(&mut self) {
        if !self.is_active {
            return;
        }

        self.input_move.x = 0.0;
        self.input_move.y = 0.0;

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) || is_key_down(KeyCode::K) {
            self.input_move.y -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) || is_key_down(KeyCode::J) {
            self.input_move.y += 1.0;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) || is_key_down(KeyCode::H) {
            self.input_move.x -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) || is_key_down(KeyCode::L) {
            self.input_move.x += 1.0;
        }
    }

    fn clear_input(&mut self) {
        self.input_move.x = 0.0;
        self.input_move.y = 0.0;
    }

    pub fn update(&mut self, dt: f32, sprite_mgr: &mut SpriteMgr, collider_mgr: &mut ColliderMgr) {
        if !self.is_active {
            return;
        }

        // Movement
        let translation_x = self.input_move.x * self.move_speed * dt;
        let translation_y = self.input_move.y * self.move_speed * dt;
        let translation = f32::Vec2::new(translation_x, translation_y);

        if translation.length_squared() > 0.0 {
            self.update_movement(&translation, sprite_mgr, collider_mgr);
        }

        // Cleanup
        self.clear_input();
    }

    fn update_movement(
        &mut self,
        translation: &f32::Vec2,
        sprite_mgr: &mut SpriteMgr,
        collider_mgr: &mut ColliderMgr,
    ) {
        let sprite_i = self.sprite_i.unwrap();
        let collider_i = self.collider_i.unwrap();

        let current_position = sprite_mgr.position[sprite_i];
        let mut new_position = current_position + *translation;

        let mov_hits =
            collider_mgr.intersect_bbox(collider_i, &translation, &mut self.movement_hit_buffer);

        if mov_hits > 0 {
            for i in 0..mov_hits {
                let hit = &self.movement_hit_buffer[i];
                if hit.is_colliding {
                    new_position -= hit.delta;
                }
            }
        }
        sprite_mgr.position[sprite_i].x = new_position.x;
        sprite_mgr.position[sprite_i].y = new_position.y;
        collider_mgr.set_position(collider_i, new_position.x, new_position.y);
    }
}
