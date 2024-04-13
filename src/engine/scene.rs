use std::collections::HashMap;

use macroquad::math::{f32, IVec2};
use tiled;

use crate::{engine::logging::log, file};

use super::{sprite::Texture2dMgr, tile::TileMgr};

const MAX_SCENE_COUNT: usize = 32;
const MAX_TILE_COUNT: usize = 32768;
const TILE_RENDERER_CACHE_SIZE: usize = 4096;

/// Loads the game scenes using Tiled.
/// The scene format is comprised of tile layers and object layers.
///
/// ## Tile layers
/// Tile layers contains graphic tiles that this same scene manager is in charge of rendering.
/// The manager keeps the `tile_id` which can be mapped to a corresponding texture using the
/// `TileManager` for rendering at the `position` which is also stored here.
///
/// ## Object layers
/// Object layers contains `object_name` and `object_class` alongside the corresponding `position`.
/// Specific game logic code (for example the `Player` manager) should read from here and
/// use the data as required.
pub struct SceneMgr {
    pub scene_id: Vec<usize>,
    /// Maps scene names to scene ids.
    pub scene_map: HashMap<String, usize>,

    // Common fields
    pub layer_id: Vec<u32>,
    pub layer_tag: Vec<LayerTag>,

    // Object layer fields
    pub object_name: Vec<Option<String>>,
    pub object_class: Vec<Option<String>>,
    pub object_position: Vec<Option<f32::Vec2>>,
    pub object_size: Vec<Option<f32::Vec2>>,
    pub object_properties: Vec<Option<tiled::Properties>>,

    // Tile layer fields
    pub tile_id: Vec<Option<u32>>,
    pub tileset_id: Vec<Option<usize>>,
    pub tile_position: Vec<Option<IVec2>>,
    pub tile_size: Vec<Option<f32::Vec2>>,

    // Manager properties
    pub loader: Option<tiled::Loader<tiled::DefaultResourceCache, TiledCursorReader>>,
    pc_assets_folder: Option<String>,

    // Active scene
    pub active_scene_id: Option<usize>,
    /// Keeps active object indices
    pub active_objects: Vec<usize>,
    /// Keeps tiles to render
    tile_renderer_cache: Vec<CachedTile>,
}

impl SceneMgr {
    pub fn new() -> Self {
        let scene_id = Vec::with_capacity(MAX_TILE_COUNT);
        let scene_map = HashMap::with_capacity(MAX_SCENE_COUNT);

        let layer_id = Vec::with_capacity(MAX_TILE_COUNT);
        let layer_tag = Vec::with_capacity(MAX_TILE_COUNT);

        let object_name = Vec::with_capacity(MAX_TILE_COUNT);
        let object_class = Vec::with_capacity(MAX_TILE_COUNT);
        let object_position = Vec::with_capacity(MAX_TILE_COUNT);
        let object_size = Vec::with_capacity(MAX_TILE_COUNT);
        let object_properties = Vec::with_capacity(MAX_TILE_COUNT);

        let tile_id = Vec::with_capacity(MAX_TILE_COUNT);
        let tileset_id = Vec::with_capacity(MAX_TILE_COUNT);
        let tile_position = Vec::with_capacity(MAX_TILE_COUNT);
        let tile_size = Vec::with_capacity(MAX_TILE_COUNT);

        let pc_assets_folder = None;

        let active_scene_id = None;
        let active_objects = Vec::with_capacity(TILE_RENDERER_CACHE_SIZE);
        let tile_renderer_cache = Vec::with_capacity(TILE_RENDERER_CACHE_SIZE);

        let loader = None;

        Self {
            scene_id,
            scene_map,

            layer_id,
            layer_tag,

            object_name,
            object_class,
            object_position,
            object_size,
            object_properties,

            tile_id,
            tileset_id,
            tile_position,
            tile_size,

            loader,
            pc_assets_folder,

            active_scene_id,
            tile_renderer_cache,
            active_objects,
        }
    }

    pub async fn init(&mut self, pc_assets_folder: Option<String>) {
        self.pc_assets_folder = pc_assets_folder;

        self.loader = Some(create_tiled_cursor_loader(self.pc_assets_folder.clone()));
    }

    pub fn len(&self) -> usize {
        self.scene_id.len()
    }

    fn add_tile(
        &mut self,
        scene_id: usize,
        layer_id: u32,
        tile_position: IVec2,
        tile_size: f32::Vec2,
        tile_id: u32,
        tileset_id: usize,
    ) -> usize {
        self.scene_id.push(scene_id);

        self.layer_id.push(layer_id);
        self.layer_tag.push(LayerTag::Tiles);

        self.object_name.push(None);
        self.object_class.push(None);
        self.object_position.push(None);
        self.object_size.push(None);
        self.object_properties.push(None);

        self.tile_id.push(Some(tile_id));
        self.tileset_id.push(Some(tileset_id));
        self.tile_position.push(Some(tile_position));
        self.tile_size.push(Some(tile_size));

        self.len() - 1
    }

    fn add_object(
        &mut self,
        scene_id: usize,
        layer_id: u32,
        object_name: String,
        object_class: String,
        object_position: f32::Vec2,
        object_size: Option<f32::Vec2>,
        object_properties: tiled::Properties,
    ) -> usize {
        self.scene_id.push(scene_id);

        self.layer_id.push(layer_id);
        self.layer_tag.push(LayerTag::Objects);

        self.object_name.push(Some(object_name));
        self.object_class.push(Some(object_class));
        self.object_position.push(Some(object_position));
        self.object_size.push(object_size);
        self.object_properties.push(Some(object_properties));

        self.tile_id.push(None);
        self.tileset_id.push(None);

        self.len() - 1
    }

    /// Adds a new scene to the `scene_map` and returns the `scene_id`.
    /// This must be done before a new scene is loaded.
    pub fn register_scene(&mut self, scene_name: &str) -> usize {
        let new_id = self.scene_map.len();
        self.scene_map.insert(String::from(scene_name), new_id);

        new_id
    }

    /// Loads a Tile map file as a scene and retuns the `scene_id`.
    pub async fn load_scene(
        &mut self,
        path: &str,
        tile_mgr: &mut TileMgr,
        texture_mgr: &mut Texture2dMgr,
    ) -> usize {
        log::debug(format!("Loading scene: {path}"));

        let loader = self.loader.as_mut().unwrap();
        let map = loader.load_tmx_map(path).unwrap();

        // Using map file path as scene name
        let scene_id = self.register_scene(path);

        // Layers are loaded sequentially, so rendering is a matter of rendering tiles in the order they
        // had been loaded.
        for layer in map.layers() {
            match layer.layer_type() {
                tiled::LayerType::Tiles(_) => {
                    log::debug(format!("Loading tile layer \"{}\"", layer.name));
                    let tile_layer = layer.as_tile_layer().unwrap();
                    self.load_map_tile_layer(
                        &tile_layer,
                        scene_id,
                        layer.id(),
                        tile_mgr,
                        texture_mgr,
                    )
                    .await;
                }
                tiled::LayerType::Objects(_) => {
                    log::debug(format!("Loading object layer \"{}\"", layer.name));
                    let object_layer = layer.as_object_layer().unwrap();
                    self.load_map_object_layer(&object_layer, scene_id, layer.id());
                }
                tiled::LayerType::Image(_) | tiled::LayerType::Group(_) => log::warning(format!(
                    "Skipping layer \"{}\". Layer type not supported",
                    layer.name
                )),
            }
        }

        scene_id
    }

    fn load_map_object_layer<'a>(
        &mut self,
        layer: &tiled::ObjectLayer<'a>,
        scene_id: usize,
        layer_id: u32,
    ) {
        for object in layer.objects() {
            let object_name = &object.name;
            let object_class = &object.user_type;
            let object_position = f32::Vec2::new(object.x, object.y);

            // For sizes, only rectangle object shapes are supported
            let object_size = match &object.shape {
                tiled::ObjectShape::Rect { width, height } => Some(f32::Vec2::new(*width, *height)),
                tiled::ObjectShape::Point(_, _) => None,
                _ => {
                    log::error(format!(
                        "Object shape \"{:?}\" not supported",
                        &object.shape
                    ));
                    None
                }
            };

            let object_properties = object.properties.clone();

            self.add_object(
                scene_id,
                layer_id,
                object_name.to_string(),
                object_class.to_string(),
                object_position,
                object_size,
                object_properties,
            );
        }
    }

    async fn load_map_tile_layer<'a>(
        &mut self,
        layer: &tiled::TileLayer<'a>,
        scene_id: usize,
        layer_id: u32,
        tile_mgr: &mut TileMgr,
        texture_mgr: &mut Texture2dMgr,
    ) {
        for j in 0..layer.height().unwrap_or_default() {
            for i in 0..layer.width().unwrap_or_default() {
                match layer.get_tile(i as i32, j as i32) {
                    Some(tile) => {
                        let tileset = tile.get_tileset();

                        let tileset_id = match tile_mgr.tileset_map.get(&tileset.name) {
                            Some(id) => *id,
                            None => {
                                // Load tileset and return the new id
                                tile_mgr.load_tileset(&tileset, texture_mgr).await.unwrap()
                            }
                        };

                        let tile_id = tile.id();
                        let tile_position = IVec2::new(i as i32, j as i32);
                        let tile_size =
                            f32::Vec2::new(tileset.tile_width as f32, tileset.tile_height as f32);

                        self.add_tile(
                            scene_id,
                            layer_id,
                            tile_position,
                            tile_size,
                            tile_id,
                            tileset_id,
                        );
                    }
                    None => {}
                }
            }
        }
    }

    /// Sets the active scene (tiles and objects).
    ///
    /// The active scene will automatically be rendered by the `SceneManager`.
    ///
    /// - Loads the `active_objects` array with the object indices from the selected scene.
    /// - Loads the `tile_renderer_cache` with the tiles from selected scene in the order they
    ///   should be rendered at.
    pub fn set_active_scene(&mut self, scene_id: Option<usize>, tile_mgr: &TileMgr) {
        self.active_scene_id = scene_id;
        let scene_id = match scene_id {
            Some(id) => id,
            None => return,
        };

        self.tile_renderer_cache.clear();
        self.active_objects.clear();

        for i in 0..self.len() {
            if self.scene_id[i] != scene_id {
                continue;
            }

            match self.layer_tag[i] {
                LayerTag::Tiles => {
                    // Load tile into the cache
                    let tilemap_id = self.tileset_id[i].unwrap();
                    let tile_id = self.tile_id[i].unwrap();
                    // TODO: cache tile_i as a reference (self.tile_i)
                    let tile_i = tile_mgr.get_tile_index(tilemap_id, tile_id);
                    let texture_i = tile_mgr.texture_i[tile_i];

                    let tile_position = self.tile_position[i].unwrap();
                    let tile_size = self.tile_size[i].unwrap();
                    let render_position = f32::Vec2::new(
                        tile_position.x as f32 * tile_size.x,
                        tile_position.y as f32 * tile_size.y,
                    );

                    self.tile_renderer_cache.push(CachedTile {
                        texture_i,
                        position: render_position,
                    });
                }

                LayerTag::Objects => {
                    // Load objects into cache `active_objects`
                    self.active_objects.push(i);
                }

                _ => continue,
            }
        }
    }

    pub fn render(&self, texture_mgr: &Texture2dMgr) {
        for cached_tile in self.tile_renderer_cache.iter() {
            texture_mgr.render_texture_unscaled(cached_tile.texture_i, cached_tile.position);
        }
    }

    // TODO: generalize get_object_property methods
    pub fn get_object_property_string(&self, index: usize, property_name: &str) -> Option<String> {
        let value: Option<String> = match self.object_properties[index].as_ref() {
            Some(property) => match property.get(property_name) {
                Some(property_value) => match property_value {
                    tiled::PropertyValue::StringValue(value) => Some(value.clone()),
                    _ => {
                        log::error(format!(
                            "Property `{:?}` of object `{:?}` was requested as a string and has another type",
                            &property_name,
                            &self.object_name[index]
                        ));
                        None
                    }
                },
                None => {
                    log::error(format!(
                        "Property `{:?}` not found for object `{:?}`",
                        &property_name, &self.object_name[index]
                    ));
                    None
                }
            },
            None => {
                log::error(format!(
                    "Object `{:?}` has no properties",
                    &self.object_name[index]
                ));
                None
            }
        };

        value
    }

    pub fn get_object_property_float(&self, index: usize, property_name: &str) -> Option<f32> {
        let value: Option<f32> = match self.object_properties[index].as_ref() {
            Some(property) => match property.get(property_name) {
                Some(property_value) => match property_value {
                    tiled::PropertyValue::FloatValue(value) => Some(*value),
                    _ => {
                        log::error(format!(
                            "Property `{:?}` of object `{:?}` was requested as a float and has another type",
                            &property_name,
                            &self.object_name[index]
                        ));
                        None
                    }
                },
                None => {
                    log::error(format!(
                        "Property `{:?}` not found for object `{:?}`",
                        &property_name, &self.object_name[index]
                    ));
                    None
                }
            },
            None => {
                log::error(format!(
                    "Object `{:?}` has no properties",
                    &self.object_name[index]
                ));
                None
            }
        };

        value
    }

    pub fn get_object_property_bool(&self, index: usize, property_name: &str) -> Option<bool> {
        let value: Option<bool> = match self.object_properties[index].as_ref() {
            Some(property) => match property.get(property_name) {
                Some(property_value) => match property_value {
                    tiled::PropertyValue::BoolValue(value) => Some(*value),
                    _ => {
                        log::error(format!(
                            "Property `{:?}` of object `{:?}` was requested as a bool and has another type",
                            &property_name,
                            &self.object_name[index]
                        ));
                        None
                    }
                },
                None => {
                    log::error(format!(
                        "Property `{:?}` not found for object `{:?}`",
                        &property_name, &self.object_name[index]
                    ));
                    None
                }
            },
            None => {
                log::error(format!(
                    "Object `{:?}` has no properties",
                    &self.object_name[index]
                ));
                None
            }
        };

        value
    }
}

fn create_tiled_cursor_loader(
    pc_assets_folder: Option<String>,
) -> tiled::Loader<tiled::DefaultResourceCache, TiledCursorReader> {
    let loader = tiled::Loader::with_cache_and_reader(
        tiled::DefaultResourceCache::new(),
        TiledCursorReader::new(pc_assets_folder),
    );

    loader
}

struct CachedTile {
    /// Texture index in `Texture2dMgr`
    texture_i: usize,
    /// Render position
    position: f32::Vec2,
}

/// General reader for tiled maps and tilesets. Can be used in WebGL.
pub struct TiledCursorReader {
    pc_assets_folder: Option<String>,
}
impl TiledCursorReader {
    fn new(pc_assets_folder: Option<String>) -> Self {
        TiledCursorReader { pc_assets_folder }
    }
}

impl tiled::ResourceReader for TiledCursorReader {
    type Resource = std::io::Cursor<Vec<u8>>;
    type Error = std::io::Error;

    fn read_from(
        &mut self,
        path: &std::path::Path,
    ) -> std::result::Result<Self::Resource, Self::Error> {
        let file = file::load_file_sync(path.to_str().unwrap(), self.pc_assets_folder.clone());
        match file {
            Ok(file_data) => Ok(std::io::Cursor::new(file_data)),
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Map/tileset file not found.",
            )),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum LayerTag {
    Tiles,
    Objects,
    Image,
    Group,
}
