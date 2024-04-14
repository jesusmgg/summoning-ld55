use std::collections::HashMap;

const LEVEL_COUNT: usize = 32;

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
