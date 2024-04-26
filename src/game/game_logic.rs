use std::collections::HashMap;

use macroquad::input::is_key_pressed;

use crate::engine::{scene::SceneMgr, sprite::Texture2dMgr, tile::TileMgr};

const LEVEL_COUNT: usize = 32;

/// Main game logic controller.
/// Takes care of initial scene setup and main game state management.
pub struct GameLogic {
    state: GameState,

    // Levels
    current_level: usize,
    level_scene_i: HashMap<usize, usize>,

    // Ingame
    turn_counter: usize,
}

/// Main game logic controller.
impl GameLogic {
    pub fn new() -> Self {
        let state = GameState::Initialization;

        let current_level = 0;
        let level_scene_i = HashMap::with_capacity(LEVEL_COUNT);

        let turn_counter = 0;

        Self {
            state,
            current_level,
            level_scene_i,
            turn_counter,
        }
    }

    pub async fn init(
        &mut self,
        scene_mgr: &mut SceneMgr,
        tile_mgr: &mut TileMgr,
        texture_mgr: &mut Texture2dMgr,
    ) {
        self.level_scene_i.insert(
            0,
            scene_mgr
                .load_scene("maps/world01.tmx", tile_mgr, texture_mgr)
                .await,
        );

        self.level_scene_i.insert(
            1,
            scene_mgr
                .load_scene("maps/world02.tmx", tile_mgr, texture_mgr)
                .await,
        );

        self.load_level(0, scene_mgr, tile_mgr);
        self.set_state(GameState::IngameGameplay);
    }

    pub fn input(&mut self, scene_mgr: &mut SceneMgr, tile_mgr: &mut TileMgr) {
        if is_key_pressed(macroquad::input::KeyCode::Key1) {
            self.load_level(0, scene_mgr, tile_mgr);
        } else if is_key_pressed(macroquad::input::KeyCode::Key2) {
            self.load_level(1, scene_mgr, tile_mgr);
        }
    }

    pub fn load_level(&mut self, level: usize, scene_mgr: &mut SceneMgr, tile_mgr: &mut TileMgr) {
        self.current_level = level;
        scene_mgr.set_active_scene(Some(self.current_scene_i()), &tile_mgr);
    }

    pub fn set_state(&mut self, state: GameState) {
        self.state = state;
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn get_level_scene_i(&self, level: &usize) -> usize {
        self.level_scene_i[level]
    }

    pub fn current_scene_i(&self) -> usize {
        self.get_level_scene_i(&self.current_level)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Initialization,
    MainMenu,
    IngameGameplay,
    IngamePause,
}
