use raylib::prelude::*;

pub struct Textures {
    pub texture_static: Texture2D,
    pub texture_1thruster: Texture2D,
    pub texture_3thruster: Texture2D,
    pub heart_texture: Texture2D,
    pub projectile_texture: Texture2D,
    pub asteroid_texture: Texture2D,
}

impl Textures {
    pub fn init(
        texture_static: Texture2D,
        texture_1thruster: Texture2D,
        texture_3thruster: Texture2D,
        heart_texture: Texture2D,
        projectile_texture: Texture2D,
        asteroid_texture: Texture2D,
    ) -> Self {
        Self {
            texture_static,
            texture_1thruster,
            texture_3thruster,
            heart_texture,
            projectile_texture,
            asteroid_texture,
        }
    }
}
