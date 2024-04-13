use std::sync::{Arc, Mutex};

use super::logging::log;

/// Load file from the path and block until its loaded
/// Will use filesystem on PC and do http request on web
///
/// Wraps over macroquad's async `load_file` function.
pub async fn load_file_async(path: &str) -> Result<Vec<u8>, macroquad::Error> {
    log::debug(format!("Loading file (async)  : {path}"));
    macroquad::file::load_file(path).await
}

/// Load file from the path and block until its loaded
/// Will use filesystem on PC and do http request on web
///
/// Based on macroquad's `load_file` function.
pub fn load_file_sync(
    path: &str,
    pc_assets_folder: Option<String>,
) -> Result<Vec<u8>, macroquad::Error> {
    fn load_file_inner(path: &str) -> Result<Vec<u8>, macroquad::Error> {
        let contents = Arc::new(Mutex::new(None));
        let path = path.to_owned();

        {
            let contents = contents.clone();
            let err_path = path.clone();

            macroquad::miniquad::fs::load_file(&path, move |bytes| {
                *contents.lock().unwrap() =
                    Some(bytes.map_err(|kind| macroquad::Error::FileError {
                        kind,
                        path: err_path.clone(),
                    }));
            });
        }

        let c = contents.lock().unwrap().take().unwrap();
        c
    }

    log::debug(format!("Loading file (sync)   : {path}"));

    #[cfg(target_os = "ios")]
    let _ = std::env::set_current_dir(std::env::current_exe().unwrap().parent().unwrap());

    #[cfg(not(target_os = "android"))]
    let path = if let Some(ref pc_assets) = pc_assets_folder {
        format!("{}/{}", pc_assets, path)
    } else {
        path.to_string()
    };

    load_file_inner(&path)
}

/// Wrapper over macroquad's `set_pc_assets_folder`.
pub fn set_pc_assets_folder(path: &str) -> Option<String> {
    macroquad::file::set_pc_assets_folder(path);
    Some(path.to_string())
}
