#![feature(plugin)]

extern crate freetype;
#[macro_use(uniform)]
extern crate glium;
#[plugin]
extern crate glium_macros;
extern crate glutin;
extern crate graphics;
extern crate image;

pub use glyph_cache::{GlyphCache, GlyphTexture};
pub use backend::{GliumSurfaceBackEnd, DrawTexture};

mod glyph_cache;
mod backend;
mod shader;
