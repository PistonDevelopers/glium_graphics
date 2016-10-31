#![deny(missing_docs)]

//! A Piston 2D graphics back-end using Glium.

extern crate rusttype;
#[macro_use(uniform, implement_vertex)]
extern crate glium;
extern crate graphics;
extern crate image;
extern crate shader_version;
extern crate shaders_graphics2d as shaders;
extern crate texture;

pub use shader_version::OpenGL;

pub use window::GliumWindow;

pub use glyph_cache::GlyphCache;
pub use back_end::{ Glium2d, GliumGraphics };
pub use texture::*;
pub use glium_texture::{ Flip, Texture };

mod glyph_cache;
mod back_end;
mod window;
mod draw_state;
mod glium_texture;
