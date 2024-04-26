use crate::engine::{
    collision::collider::ColliderMgr,
    scene::SceneMgr,
    sprite::{SpriteMgr, Texture2dMgr},
};

use macroquad::math::f32;

const MAX_OBJECTS: usize = 512;

pub struct SummoningCircleMgr {
    is_active: Vec<bool>,

    scene_object_i: Vec<usize>,
    collider_i: Vec<usize>,
    sprite_i: Vec<usize>,
}

// TODO: make generic object manager for scenes instead of duplicating most of this code
impl SummoningCircleMgr {
    pub fn new() -> Self {
        Self {
            is_active: Vec::with_capacity(MAX_OBJECTS),

            scene_object_i: Vec::with_capacity(MAX_OBJECTS),
            collider_i: Vec::with_capacity(MAX_OBJECTS),
            sprite_i: Vec::with_capacity(MAX_OBJECTS),
        }
    }

    pub fn add(&mut self, sprite_i: usize, collider_i: usize, scene_object_i: usize) -> usize {
        self.is_active.push(false);

        self.scene_object_i.push(scene_object_i);
        self.collider_i.push(collider_i);
        self.sprite_i.push(sprite_i);

        self.len() - 1
    }

    pub async fn add_from_scene_object(
        &mut self,
        position: f32::Vec2,
        scene_object_i: usize,
        collider_mgr: &mut ColliderMgr,
        sprite_mgr: &mut SpriteMgr,
        texture_mgr: &mut Texture2dMgr,
    ) -> usize {
        // Create sprite
        let sprite_i = sprite_mgr
            .add_from_file(
                "sprites/summoning_circle.png",
                position,
                f32::Vec2 { x: 0.05, y: 0.05 },
                texture_mgr,
            )
            .await;

        // Create collider
        let collider_i = collider_mgr.add_from_sprite(sprite_i, None, sprite_mgr);
        collider_mgr.render_bbox[collider_i] = false;

        self.add(sprite_i, collider_i, scene_object_i)
    }

    pub fn len(&self) -> usize {
        self.collider_i.len()
    }

    fn set_active(
        &mut self,
        index: usize,
        is_active: bool,
        collider_mgr: &mut ColliderMgr,
        sprite_mgr: &mut SpriteMgr,
    ) {
        self.is_active[index] = is_active;
        collider_mgr.set_active(self.collider_i[index], is_active);
        sprite_mgr.set_active(self.sprite_i[index], is_active);
    }

    pub async fn spawn(
        &mut self,
        scene_mgr: &SceneMgr,
        collider_mgr: &mut ColliderMgr,
        sprite_mgr: &mut SpriteMgr,
        texture_mgr: &mut Texture2dMgr,
    ) {
        if scene_mgr.active_scene_id == None || scene_mgr.active_objects.len() == 0 {
            return;
        }

        'scene_iter: for scene_object_i in &scene_mgr.active_objects {
            if scene_mgr.object_class[*scene_object_i].as_ref().unwrap() == "SummoningCircle" {
                // Existing in manager, activate it
                for index in 0..self.len() {
                    if self.scene_object_i[index] == *scene_object_i {
                        self.set_active(index, true, collider_mgr, sprite_mgr);
                        continue 'scene_iter;
                    }
                }

                // New object, create it
                let position = scene_mgr.object_position[*scene_object_i].unwrap();
                let new_index = self
                    .add_from_scene_object(
                        position,
                        *scene_object_i,
                        collider_mgr,
                        sprite_mgr,
                        texture_mgr,
                    )
                    .await;
                self.set_active(new_index, true, collider_mgr, sprite_mgr);
            }
        }
    }

    pub fn despawn(
        &mut self,
        scene_mgr: &SceneMgr,
        collider_mgr: &mut ColliderMgr,
        sprite_mgr: &mut SpriteMgr,
    ) {
        'scene_iter: for scene_object_i in &scene_mgr.objects_to_despawn {
            if scene_mgr.object_class[*scene_object_i].as_ref().unwrap() == "SummoningCircle" {
                for index in 0..self.len() {
                    if self.scene_object_i[index] == *scene_object_i && self.is_active(index) {
                        self.set_active(index, false, collider_mgr, sprite_mgr);
                        continue 'scene_iter;
                    }
                }
            }
        }
    }

    pub fn is_active(&self, index: usize) -> bool {
        self.is_active[index]
    }

    pub fn collider_i(&self, index: usize) -> usize {
        self.collider_i[index]
    }
}
