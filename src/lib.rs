#![deny(missing_docs)]

//! A Piston 2D graphics back-end using Glium.

#[macro_use(uniform, implement_vertex)]
extern crate glium;
extern crate graphics;
extern crate shader_version;
extern crate shaders_graphics2d as shaders;
extern crate texture;

pub use shader_version::OpenGL;

#[cfg(feature = "glium_window")]
pub use window::GliumWindow;

/// Stores textures for text rendering.
pub type GlyphCache<'a, F> = graphics::glyph_cache::rusttype::GlyphCache<'a, F, Texture>;

pub use back_end::{ Glium2d, GliumGraphics };
pub use texture::*;
pub use glium_texture::{ Flip, Texture };

mod back_end;
#[cfg(feature = "glium_window")]
mod window;
mod draw_state;
mod glium_texture;
