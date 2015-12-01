#![deny(missing_docs)]

//! A Piston 2D graphics back-end using Glium.

extern crate freetype;
#[macro_use(uniform, implement_vertex)]
extern crate glium;
extern crate graphics;
extern crate image;
extern crate shader_version;
extern crate shaders_graphics2d as shaders;
extern crate texture;

pub use window::GliumWindow;

pub use glyph_cache::GlyphCache;
pub use back_end::{ Flip, Glium2d, GliumGraphics, Texture };
pub use texture::*;

mod glyph_cache;
mod back_end;
mod window;
mod draw_state;
