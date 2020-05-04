use bevy_asset::{self, Handle};
use bevy_derive::Uniforms;
use bevy_render::{colors, texture::Texture, Color};

#[derive(Uniforms)]
#[module(meta = false)]
pub struct ColorMaterial {
    pub color: Color,
    #[uniform(shader_def)]
    pub texture: Option<Handle<Texture>>,
}

impl ColorMaterial {
    pub fn color(color: Color) -> Self {
        ColorMaterial {
            color,
            texture: None,
        }
    }

    pub fn texture(texture: Handle<Texture>) -> Self {
        ColorMaterial {
            color: colors::WHITE,
            texture: Some(texture),
        }
    }

    pub fn modulated_texture(texture: Handle<Texture>, color: Color) -> Self {
        ColorMaterial {
            color,
            texture: Some(texture),
        }
    }
}

impl Default for ColorMaterial {
    fn default() -> Self {
        ColorMaterial {
            color: Color::rgb(1.0, 1.0, 1.0),
            texture: None,
        }
    }
}

impl From<Color> for ColorMaterial {
    fn from(color: Color) -> Self {
        ColorMaterial::color(color)
    }
}

impl From<Handle<Texture>> for ColorMaterial {
    fn from(texture: Handle<Texture>) -> Self {
        ColorMaterial::texture(texture)
    }
}