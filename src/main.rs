mod engine;
mod game;
mod game_manager;

use engine::{
    file,
    logging::log::{set_active_log_level, LogLevel},
};
use game_manager::GameMgr;
use macroquad::window::next_frame;

#[macroquad::main("summoning-ld55")]
async fn main() {
    set_active_log_level(LogLevel::DEBUG);

    let pc_assets_folder = file::set_pc_assets_folder("assets");

    let mut game_mgr = GameMgr::new(pc_assets_folder);

    game_mgr.init().await;

    loop {
        game_mgr.despawn();
        game_mgr.spawn().await;

        game_mgr.input();
        game_mgr.update();
        game_mgr.render();

        next_frame().await;
    }
}
