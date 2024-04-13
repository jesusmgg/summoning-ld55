use std::collections::HashMap;

use super::{logging::log, scene::TiledCursorReader, sprite::Texture2dMgr};

const MAX_TILE_COUNT: usize = 1024;
const MAX_TILESET_COUNT: usize = 128;

pub struct TileMgr {
    pub tile_id: Vec<u32>,
    pub tileset_id: Vec<usize>,

    /// Maps tileset names to tileset ids.
    pub tileset_map: HashMap<String, usize>,
    /// Maps (`tileset_id`, `tile_id`) tuples to tile manager indices.
    tile_map: HashMap<(usize, u32), usize>,

    pub texture_i: Vec<usize>,
}

impl TileMgr {
    pub fn new() -> Self {
        let tile_id = Vec::with_capacity(MAX_TILE_COUNT);
        let tileset_id = Vec::with_capacity(MAX_TILE_COUNT);

        let tileset_map = HashMap::with_capacity(MAX_TILESET_COUNT);
        let tile_map = HashMap::with_capacity(MAX_TILE_COUNT);

        let texture_i = Vec::with_capacity(MAX_TILE_COUNT);

        Self {
            tile_id,
            tileset_id,
            tileset_map,
            tile_map,
            texture_i,
        }
    }

    /// Adds a new tile to the manager. For performance reasons, the tileset must be registered
    /// with `register_tileset` before adding tiles to it.
    pub fn add(&mut self, tile_id: u32, tileset_id: usize, texture_i: usize) -> usize {
        self.tile_id.push(tile_id);
        self.tileset_id.push(tileset_id);

        self.texture_i.push(texture_i);

        let index = self.len() - 1;

        self.tile_map.insert((tileset_id, tile_id), index);

        index
    }

    /// Adds a new tileset to the `tileset_map` and returns the `tileset_id`.
    /// This must be done before a new tileset is loaded.
    pub fn register_tileset(&mut self, tileset_name: &str) -> usize {
        let new_id = self.tileset_map.len();
        self.tileset_map.insert(String::from(tileset_name), new_id);

        new_id
    }

    pub fn len(&self) -> usize {
        self.tile_id.len()
    }

    pub fn get_tile_index(&self, tileset_id: usize, tile_id: u32) -> usize {
        let tile_i = self.tile_map.get(&(tileset_id, tile_id)).unwrap();

        *tile_i
    }

    /// Loads every tile from a Tiled tileset. Also registers the tileset.
    /// Returns the loaded tileset id.
    pub async fn load_tileset(
        &mut self,
        tileset: &tiled::Tileset,
        texture_mgr: &mut Texture2dMgr,
    ) -> Result<usize, std::io::Error> {
        log::debug(format!("Loading tileset: {}", &tileset.name));

        let error_str = format!("Can't load tileset {}: it contains no tiles", tileset.name);
        if tileset.tilecount <= 0 {
            log::error(&error_str);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, error_str));
        }

        let tileset_id = self.register_tileset(&tileset.name);

        for tile in tileset.tiles() {
            let tile_id = tile.0;
            let image_path = &tile
                .1
                .image
                .as_ref()
                .unwrap()
                .source
                .as_os_str()
                .to_str()
                .unwrap();

            log::debug(format!("Loading tile #{tile_id} from {image_path:?}"));

            let texture_i = texture_mgr.add_from_file(&image_path).await;

            self.add(tile_id, tileset_id, texture_i);
        }

        Ok(tileset_id)
    }

    /// Loads every tile from a file path. Also registers the tileset.
    /// Returns the loaded tileset id.
    pub async fn load_tileset_from_path(
        &mut self,
        path: &str,
        loader: &mut tiled::Loader<tiled::DefaultResourceCache, TiledCursorReader>,
        texture_mgr: &mut Texture2dMgr,
    ) -> Result<usize, std::io::Error> {
        let tileset = loader.load_tsx_tileset(path).unwrap();
        self.load_tileset(&tileset, texture_mgr).await
    }
}
