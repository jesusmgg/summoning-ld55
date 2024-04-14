use macroquad::{
    color,
    math::f32,
    texture::{draw_texture_ex, load_texture, DrawTextureParams, Texture2D},
};

use super::logging::log;

const MAX_SPRITE_COUNT: usize = 1024;
const MAX_TEXTURE_COUNT: usize = 256;

/// Holds references to sprite instances in the game.
pub struct SpriteMgr {
    position: Vec<f32::Vec2>,
    size: Vec<f32::Vec2>,
    scale: Vec<f32::Vec2>,
    is_active: Vec<bool>,

    // References
    pub texture_i: Vec<usize>,

    scaled_size_cache: Vec<f32::Vec2>,
}

impl SpriteMgr {
    pub fn new() -> Self {
        let position = Vec::with_capacity(MAX_SPRITE_COUNT);
        let size = Vec::with_capacity(MAX_SPRITE_COUNT);
        let scale = Vec::with_capacity(MAX_SPRITE_COUNT);
        let is_active = Vec::with_capacity(MAX_SPRITE_COUNT);

        let texture_i = Vec::with_capacity(MAX_SPRITE_COUNT);

        let scaled_size_cache = Vec::with_capacity(MAX_SPRITE_COUNT);

        Self {
            position,
            size,
            scale,
            is_active,

            texture_i,

            scaled_size_cache,
        }
    }

    /// Adds a sprite instance with the provided texture reference index.
    pub fn add(
        &mut self,
        texture_i: usize,
        position: f32::Vec2,
        size: f32::Vec2,
        scale: f32::Vec2,
    ) -> usize {
        self.position.push(position);
        self.size.push(size);
        self.scale.push(scale);
        self.scaled_size_cache.push(size * scale);
        self.is_active.push(true);

        self.texture_i.push(texture_i);

        self.len() - 1
    }

    /// Adds a sprite instance and adds a new texture from the provided file path.
    pub async fn add_from_file(
        &mut self,
        file_path: &str,
        position: f32::Vec2,
        scale: f32::Vec2,

        texture_mgr: &mut Texture2dMgr,
    ) -> usize {
        let texture_i = texture_mgr.add_from_file(file_path).await;
        let size = texture_mgr.texture[texture_i].size();

        self.add(texture_i, position, size, scale)
    }

    pub fn len(&self) -> usize {
        self.position.len()
    }

    pub fn size(&self, index: usize) -> &f32::Vec2 {
        &self.size[index]
    }

    pub fn set_scale(&mut self, index: usize, scale: f32::Vec2) {
        self.scale[index].x = scale.x;
        self.scale[index].y = scale.y;

        self.scaled_size_cache[index] = self.size[index] * scale;
    }

    pub fn scale(&self, index: usize) -> &f32::Vec2 {
        &self.scale[index]
    }

    pub fn scaled_size(&self, index: usize) -> &f32::Vec2 {
        &self.scaled_size_cache[index]
    }

    /// Render every active sprite
    pub fn render(&self, texture_mgr: &Texture2dMgr) {
        for i in 0..self.len() {
            if !self.is_active[i] {
                continue;
            }

            texture_mgr.render_texture(self.texture_i[i], self.position[i], self.scale[i]);
        }
    }

    pub fn position(&self, index: usize) -> f32::Vec2 {
        self.position[index]
    }

    pub fn set_position(&mut self, index: usize, position: f32::Vec2) {
        self.position[index] = position;
    }

    pub fn translate(&mut self, index: usize, delta: f32::Vec2) {
        self.set_position(index, self.position[index] + delta);
    }

    pub fn set_active(&mut self, index: usize, is_active: bool) {
        self.is_active[index] = is_active
    }

    pub fn is_active(&self, index: usize) -> bool {
        self.is_active[index]
    }
}

/// Holds reference to all textures used by the game.
pub struct Texture2dMgr {
    pub texture: Vec<Texture2D>,

    pub is_atlas_outdated: bool,
}

impl Texture2dMgr {
    pub fn new() -> Self {
        let texture = Vec::with_capacity(MAX_TEXTURE_COUNT);

        let is_atlas_outdated = false;

        Self {
            texture,

            is_atlas_outdated,
        }
    }

    pub async fn add_from_file(&mut self, file_path: &str) -> usize {
        log::debug(format!("Loading texture: {file_path}"));
        let texture = load_texture(file_path).await.unwrap();
        texture.set_filter(macroquad::texture::FilterMode::Nearest);

        self.texture.push(texture);

        self.is_atlas_outdated = true;

        self.len() - 1
    }

    pub fn len(&self) -> usize {
        self.texture.len()
    }

    pub fn update(&mut self) {
        if self.is_atlas_outdated {
            self.rebuild_texture_atlas();
        }
    }

    pub fn rebuild_texture_atlas(&self) {
        // TODO: update macroquad when the fix is released and uncomment the code below.
        // build_textures_atlas();
    }

    pub fn render_texture(&self, index: usize, position: f32::Vec2, scale: f32::Vec2) {
        let size = &self.texture[index].size();
        let scaled_size = f32::Vec2 {
            x: size.x * scale.x,
            y: size.y * scale.y,
        };

        let params = DrawTextureParams {
            dest_size: Some(scaled_size),
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };

        draw_texture_ex(
            &self.texture[index],
            position.x,
            position.y,
            color::WHITE,
            params,
        );
    }

    pub fn render_texture_unscaled(&self, index: usize, position: f32::Vec2) {
        let params = DrawTextureParams {
            dest_size: None,
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };

        draw_texture_ex(
            &self.texture[index],
            position.x,
            position.y,
            color::WHITE,
            params,
        );
    }
}
