mod engine;
mod game;
mod game_manager;

use engine::{
    file,
    logging::log::{set_active_log_level, LogLevel},
};
use game_manager::GameMgr;
use macroquad::window::next_frame;

#[macroquad::main("Pipe Warp")]
async fn main() {
    set_active_log_level(LogLevel::DEBUG);

    let pc_assets_folder = file::set_pc_assets_folder("assets");

    let mut game_mgr = GameMgr::new(pc_assets_folder);
    game_mgr.init().await;

    // ###########################################
    // ####### Game scene setup start
    // ###########################################
    let scene_id = game_mgr
        .scene_mgr
        .load_scene(
            "maps/future.tmx",
            &mut game_mgr.tile_mgr,
            &mut game_mgr.texture2d_mgr,
        )
        .await;

    game_mgr
        .scene_mgr
        .set_active_scene(Some(scene_id), &game_mgr.tile_mgr);
    // ###########################################
    // ####### Game scene setup end
    // ###########################################

    game_mgr.spawn();

    loop {
        game_mgr.input();
        game_mgr.update();
        game_mgr.render();

        next_frame().await;
    }
}
