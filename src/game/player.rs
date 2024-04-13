use crate::{
    engine::collision::collider::{self, ColliderMgr, Hit},
    engine::scene::SceneMgr,
    engine::sprite::{SpriteMgr, Texture2dMgr},
};
use macroquad::math::f32;
use macroquad::text::draw_text;
use macroquad::{
    color,
    input::{is_key_down, is_mouse_button_pressed, mouse_position, KeyCode, MouseButton},
    window::screen_height,
};

const MAX_UNIT_COUNT: usize = 1024;
const MOVE_DISTANCE_TOLERANCE: f32 = 1.0;

pub struct PlayerUnitMgr {
    move_speed: Vec<f32>,
    /// Current movement input
    input_move: Vec<f32::Vec2>,
    is_selected: Vec<bool>,
    is_active: Vec<bool>,
    team: Vec<PlayerTeam>,

    collider_i: Vec<Option<usize>>,
    sprite_i: Vec<Option<usize>>,

    /// Collision hit buffers
    movement_hit_buffer: Vec<Vec<Hit>>,
    selection_hit: Vec<Hit>,

    move_target: Vec<Option<f32::Vec2>>,

    mouse_pos: f32::Vec2,
}

impl PlayerUnitMgr {
    pub fn new() -> Self {
        let move_speed = Vec::with_capacity(MAX_UNIT_COUNT);
        let input_move = Vec::with_capacity(MAX_UNIT_COUNT);
        let is_selected = Vec::with_capacity(MAX_UNIT_COUNT);
        let is_active = Vec::with_capacity(MAX_UNIT_COUNT);

        let team = Vec::with_capacity(MAX_UNIT_COUNT);

        let collider_i = Vec::with_capacity(MAX_UNIT_COUNT);
        let sprite_i = Vec::with_capacity(MAX_UNIT_COUNT);

        let movement_hit_buffer = Vec::with_capacity(MAX_UNIT_COUNT);
        let selection_hit = Vec::with_capacity(MAX_UNIT_COUNT);

        let move_target = Vec::with_capacity(MAX_UNIT_COUNT);

        let mouse_pos = f32::Vec2::ZERO;

        Self {
            move_speed,
            input_move,
            is_selected,
            is_active,
            team,

            collider_i,
            sprite_i,

            movement_hit_buffer,
            selection_hit,

            move_target,

            mouse_pos,
        }
    }

    pub async fn add(
        &mut self,
        move_speed: f32,
        team: PlayerTeam,
        sprite_mgr: &mut SpriteMgr,
        collider_mgr: &mut ColliderMgr,
        texture_mgr: &mut Texture2dMgr,
    ) -> usize {
        self.move_speed.push(move_speed);
        self.input_move.push(f32::Vec2::ZERO);
        self.is_selected.push(false);
        self.is_active.push(false);
        self.move_target.push(None);

        let index = self.len() - 1;

        if index == 0 {
            assert!(
                team == PlayerTeam::Player,
                "First unit added should have 'player' team."
            );
        }

        self.team.push(team);

        // Create hit buffers
        self.movement_hit_buffer
            .push(ColliderMgr::create_hit_buffer(
                collider::MAX_COLLISION_COUNT,
            ));
        self.selection_hit.push(Hit {
            is_colliding: false,
            collider_i: usize::MAX,
            position: f32::Vec2::ZERO,
            delta: f32::Vec2::ZERO,
            normal: f32::Vec2::ZERO,
        });

        // Create sprite
        let sprite_i = sprite_mgr
            .add_from_file(
                "sprites/player01.png",
                f32::Vec2::ZERO,
                f32::Vec2 { x: 0.1, y: 0.1 },
                texture_mgr,
            )
            .await;
        self.sprite_i.push(Some(sprite_i));

        // Create collider
        let collider_i = collider_mgr.add_from_sprite(sprite_i, None, sprite_mgr);
        self.collider_i.push(Some(collider_i));
        collider_mgr.render_bbox[collider_i] = false;

        // Set not active
        self.set_active(index, false, sprite_mgr, collider_mgr);

        index
    }

    pub fn len(&self) -> usize {
        self.is_active.len()
    }

    pub async fn spawn(
        &mut self,
        scene_mgr: &SceneMgr,
        sprite_mgr: &mut SpriteMgr,
        collider_mgr: &mut ColliderMgr,
        texture_mgr: &mut Texture2dMgr,
    ) {
        if scene_mgr.active_scene_id == None || scene_mgr.active_objects.len() == 0 {
            return;
        }

        for scene_object_i in &scene_mgr.active_objects {
            let object_class = scene_mgr.object_class[*scene_object_i].as_ref();
            if object_class.unwrap() != "PlayerUnit" {
                continue;
            }
            let name = scene_mgr.object_name[*scene_object_i].as_ref().unwrap();

            let team = match scene_mgr.get_object_property_string(*scene_object_i, "team") {
                Some(team_str) => match team_str.as_str() {
                    "Player" => PlayerTeam::Player,
                    "Enemy" => PlayerTeam::Enemy,
                    _ => panic!("Invalid value for property `team` in object `{:?}`", name),
                },
                None => panic!(
                    "`team` property is required for PlayerUnit object `{:?}`",
                    name
                ),
            };

            let move_speed =
                match scene_mgr.get_object_property_float(*scene_object_i, "move_speed") {
                    Some(move_speed) => move_speed,
                    None => panic!(
                        "`move_speed` property is required for PlayerUnit object `{:?}`",
                        name
                    ),
                };

            let position = scene_mgr.object_position[*scene_object_i].unwrap();

            let new_index = self
                .add(move_speed, team, sprite_mgr, collider_mgr, texture_mgr)
                .await;

            sprite_mgr.position[self.sprite_i[new_index].unwrap()] = position;

            // if new_index == 0 {
            // self.set_active(new_index, true, sprite_mgr, collider_mgr);
            // }
            self.set_active(new_index, true, sprite_mgr, collider_mgr);
        }
    }

    fn set_active(
        &mut self,
        index: usize,
        is_active: bool,
        sprite_mgr: &mut SpriteMgr,
        collider_mgr: &mut ColliderMgr,
    ) {
        self.is_active[index] = is_active;
        sprite_mgr.is_active[self.sprite_i[index].unwrap()] = is_active;
        let position = sprite_mgr.position[index];
        collider_mgr.set_position(index, position.x, position.y);
        collider_mgr.set_active(self.collider_i[index].unwrap(), is_active);
    }

    pub fn input(&mut self, collider_mgr: &ColliderMgr) {
        (self.mouse_pos.x, self.mouse_pos.y) = mouse_position();
        let is_mouse_r_pressed = is_mouse_button_pressed(MouseButton::Right);
        let is_mouse_l_pressed = is_mouse_button_pressed(MouseButton::Left);

        for i in 0..self.len() {
            // Selection
            if !self.is_active[i] {
                continue;
            }

            if is_mouse_l_pressed {
                let mut selection_hit = &mut self.selection_hit[i];
                let collider_i = self.collider_i[i].unwrap();
                let bbox = collider_mgr.bbox[collider_i];
                let bbox_center = bbox.center();

                self.is_selected[i] = ColliderMgr::intersect_point_single(
                    &self.mouse_pos,
                    &bbox,
                    &bbox_center,
                    collider_i,
                    &mut selection_hit,
                );

                println!("is selected {}", self.is_selected[i]);
            }

            // Movement
            if !self.is_selected[i] {
                continue;
            }

            if is_mouse_r_pressed {
                self.move_target[i] = Some(self.mouse_pos);
            }
        }
    }

    fn clear_input(&mut self, index: usize) {
        self.input_move[index].x = 0.0;
        self.input_move[index].y = 0.0;
    }

    pub fn update(&mut self, dt: f32, sprite_mgr: &mut SpriteMgr, collider_mgr: &mut ColliderMgr) {
        for i in 0..self.len() {
            if !self.is_active[i] {
                continue;
            }

            if self.move_target[i] == None {
                continue;
            }

            let index_text = format!("Update unit index {}", i);
            println!("{}", index_text);

            draw_text(
                index_text.as_str(),
                0.0,
                screen_height() - 84.0,
                32.0,
                color::WHITE,
            );

            let sprite_i = self.sprite_i[i].unwrap();
            let position = sprite_mgr.position[sprite_i];
            let size = sprite_mgr.scaled_size(sprite_i);

            // Get movement vector
            let move_target = self.move_target[i].unwrap() - *size / 2.0;
            println!("move target x:{} y:{}", move_target.x, move_target.y);
            println!("position    x:{} y:{}", position.x, position.y);

            let distance = move_target - position;
            if distance.length_squared() < MOVE_DISTANCE_TOLERANCE {
                self.move_target[i] = None;
                continue;
            }

            self.input_move[i] = distance.normalize_or_zero();

            let translation_x = self.input_move[i].x * self.move_speed[i] * dt;
            let translation_y = self.input_move[i].y * self.move_speed[i] * dt;
            let translation = f32::Vec2::new(translation_x, translation_y);

            if translation.length_squared() > 0.0 {
                self.update_movement(i, &translation, sprite_mgr, collider_mgr);
            }

            println!("translation {}", translation);

            // Cleanup
            self.clear_input(i);
        }
    }

    fn update_movement(
        &mut self,
        index: usize,
        translation: &f32::Vec2,
        sprite_mgr: &mut SpriteMgr,
        collider_mgr: &mut ColliderMgr,
    ) {
        let sprite_i = self.sprite_i[index].unwrap();
        let collider_i = self.collider_i[index].unwrap();

        let current_position = sprite_mgr.position[sprite_i];
        let mut new_position = current_position + *translation;

        let mov_hits = collider_mgr.intersect_bbox(
            collider_i,
            &translation,
            &mut self.movement_hit_buffer[index],
        );

        if mov_hits > 0 {
            for i in 0..mov_hits {
                let hit = &self.movement_hit_buffer[index][i];
                if hit.is_colliding {
                    new_position -= hit.delta;
                }
            }
        }
        sprite_mgr.position[sprite_i].x = new_position.x;
        sprite_mgr.position[sprite_i].y = new_position.y;
        collider_mgr.set_position(collider_i, new_position.x, new_position.y);
    }

    pub fn render(&self, collider_mgr: &ColliderMgr) {
        for i in 0..self.len() {
            if !self.is_selected[i] || !self.is_active[i] {
                continue;
            }

            // Render selection box
            let collider_i = self.collider_i[i].unwrap();
            let bbox = collider_mgr.bbox[collider_i];
            macroquad::shapes::draw_rectangle_lines(
                bbox.x,
                bbox.y,
                bbox.w,
                bbox.h,
                4.0,
                color::YELLOW,
            );
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum PlayerTeam {
    Player,
    Enemy,
}
