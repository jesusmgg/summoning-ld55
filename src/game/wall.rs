use crate::engine::{collision::collider::ColliderMgr, scene::SceneMgr};
use macroquad::math::{f32, Rect};

const MAX_OBJECTS: usize = 256;

pub struct WallMgr {
    is_active: Vec<bool>,

    scene_object_i: Vec<usize>,
    collider_i: Vec<usize>,
}

impl WallMgr {
    pub fn new() -> Self {
        Self {
            is_active: Vec::with_capacity(MAX_OBJECTS),

            scene_object_i: Vec::with_capacity(MAX_OBJECTS),
            collider_i: Vec::with_capacity(MAX_OBJECTS),
        }
    }

    pub fn add(&mut self, collider_i: usize, scene_object_i: usize) -> usize {
        self.is_active.push(false);

        self.collider_i.push(collider_i);
        self.scene_object_i.push(scene_object_i);

        self.len() - 1
    }

    pub fn add_from_scene_object(
        &mut self,
        position: f32::Vec2,
        size: f32::Vec2,
        scene_object_i: usize,
        collider_mgr: &mut ColliderMgr,
    ) -> usize {
        let bbox = Rect::new(position.x, position.y, size.x, size.y);
        let collider_i = collider_mgr.add(bbox);

        self.add(collider_i, scene_object_i)
    }

    pub fn len(&self) -> usize {
        self.collider_i.len()
    }

    fn set_active(&mut self, index: usize, is_active: bool, collider_mgr: &mut ColliderMgr) {
        self.is_active[index] = is_active;
        collider_mgr.set_active(self.collider_i[index], is_active);
        collider_mgr.render_bbox[self.collider_i[index]] = true;
    }

    /// Reads objects with class `Wall` from the scene and spawn them. If they are already loaded,
    /// activate them, if not, create new instances.
    pub fn spawn(&mut self, scene_mgr: &SceneMgr, collider_mgr: &mut ColliderMgr) {
        if scene_mgr.active_scene_id == None || scene_mgr.active_objects.len() == 0 {
            return;
        }

        'scene_iter: for scene_object_i in &scene_mgr.active_objects {
            if scene_mgr.object_class[*scene_object_i].as_ref().unwrap() == "Wall" {
                for index in 0..self.len() {
                    if self.scene_object_i[index] == *scene_object_i {
                        self.set_active(index, true, collider_mgr);
                        continue 'scene_iter;
                    }
                }

                let position = scene_mgr.object_position[*scene_object_i].unwrap();
                // Assuming the wall object is a valid rectangle here
                let size = scene_mgr.object_size[*scene_object_i].unwrap();

                let wall_i =
                    self.add_from_scene_object(position, size, *scene_object_i, collider_mgr);
                self.set_active(wall_i, true, collider_mgr);
            }
        }
    }

    pub fn despawn(&mut self, scene_mgr: &SceneMgr, collider_mgr: &mut ColliderMgr) {
        'scene_iter: for scene_object_i in &scene_mgr.objects_to_despawn {
            if scene_mgr.object_class[*scene_object_i].as_ref().unwrap() == "Wall" {
                for index in 0..self.len() {
                    if self.scene_object_i[index] == *scene_object_i && self.is_active[index] {
                        self.set_active(index, false, collider_mgr);
                        continue 'scene_iter;
                    }
                }
            }
        }
    }
}
