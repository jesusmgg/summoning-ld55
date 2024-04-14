use crate::{
    engine::{
        camera::camera::CameraMgr,
        collision::collider::ColliderMgr,
        diagnostics::DiagnosticsMgr,
        scene::SceneMgr,
        sprite::{SpriteMgr, Texture2dMgr},
        tile::TileMgr,
    },
    game::selector_box::SelectorBox,
};

use crate::game::{player::PlayerUnitMgr, wall::WallMgr};

use macroquad::{
    color, input::is_key_pressed, miniquad::window::quit, time::get_frame_time,
    window::clear_background,
};

/// Main game manager. Owns all individual system managers.
pub struct GameMgr {
    pub texture2d_mgr: Texture2dMgr,
    pub tile_mgr: TileMgr,
    pub scene_mgr: SceneMgr,
    pub sprite_mgr: SpriteMgr,
    pub collider_mgr: ColliderMgr,
    pub camera_mgr: CameraMgr,
    pub diagnostics_mgr: DiagnosticsMgr,

    pub player_unit_mgr: PlayerUnitMgr,
    pub wall_mgr: WallMgr,
    pub selector_box: SelectorBox,

    // TODO: consider an alternative to passing around clones of the `pc_assets_folder`.
    pub pc_assets_folder: Option<String>,
}

impl GameMgr {
    pub fn new(pc_assets_folder: Option<String>) -> Self {
        let texture2d_mgr = Texture2dMgr::new();
        let tile_mgr = TileMgr::new();
        let scene_mgr = SceneMgr::new();
        let sprite_mgr = SpriteMgr::new();
        let collider_mgr = ColliderMgr::new();
        let camera_mgr = CameraMgr::new();
        let diagnostics_mgr = DiagnosticsMgr::new();

        let player_unit_mgr = PlayerUnitMgr::new();
        let wall_mgr = WallMgr::new();
        let selector_box = SelectorBox::new();

        Self {
            texture2d_mgr,
            tile_mgr,
            sprite_mgr,
            collider_mgr,
            scene_mgr,
            camera_mgr,
            diagnostics_mgr,

            player_unit_mgr,
            wall_mgr,
            selector_box,

            pc_assets_folder,
        }
    }

    pub async fn init(&mut self) {
        self.diagnostics_mgr.init();
        self.scene_mgr.init(self.pc_assets_folder.clone()).await;
        self.camera_mgr.init();

        self.selector_box.init(&mut self.collider_mgr);
        self.player_unit_mgr.init(&self.selector_box);
    }

    pub async fn spawn(&mut self) {
        self.player_unit_mgr
            .spawn(
                &self.scene_mgr,
                &mut self.sprite_mgr,
                &mut self.collider_mgr,
                &mut self.texture2d_mgr,
            )
            .await;

        self.wall_mgr.spawn(&self.scene_mgr, &mut self.collider_mgr);
    }

    pub fn input(&mut self) {
        if is_key_pressed(macroquad::prelude::KeyCode::Q) {
            quit();
        }

        self.selector_box
            .input(&self.camera_mgr, &mut self.collider_mgr);

        self.player_unit_mgr
            .input(&self.collider_mgr, &self.camera_mgr);
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        self.texture2d_mgr.update();

        self.selector_box.update(&self.collider_mgr);
        self.player_unit_mgr.update(
            dt,
            &self.selector_box,
            &mut self.sprite_mgr,
            &mut self.collider_mgr,
        );
    }

    pub fn render(&self) {
        clear_background(color::RED);

        self.scene_mgr.render(&self.texture2d_mgr);
        self.sprite_mgr.render(&self.texture2d_mgr);
        self.collider_mgr.render();

        self.selector_box.render(&self.collider_mgr);
        self.player_unit_mgr.render(&self.collider_mgr);

        self.diagnostics_mgr.render();
    }
}
