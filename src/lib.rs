#![feature(std_misc)]

extern crate freetype;
#[macro_use(uniform)]
extern crate glium;
extern crate glutin;
extern crate graphics;
extern crate image;

pub use glyph_cache::{GlyphCache, GlyphTexture};
pub use backend::{GliumBackendSystem, GliumSurfaceBackEnd, DrawTexture};

mod glyph_cache;
mod backend;
mod shader;
