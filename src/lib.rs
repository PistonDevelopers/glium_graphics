#![feature(std_misc)]

extern crate freetype;
extern crate graphics;
extern crate shader_version;
extern crate "texture" as texture_lib;
#[macro_use(uniform, implement_vertex)]
extern crate "glium" as glium_lib;

pub mod glium {
    pub use glium_lib::{ DisplayBuild, Surface };
}

pub use glyph_cache::{ GlyphCache, GlyphTexture };
pub use backend::{ Glium2d, GliumGraphics };
pub use shader_version::OpenGL;
pub use backend::Glium2d as GliumBackendSystem;
pub use backend::GliumGraphics as GliumSurfaceBackEnd;
pub use texture::GliumTexture;
pub use texture_lib::TextureWithDevice;

mod glyph_cache;
mod backend;
mod shader;
mod texture;
