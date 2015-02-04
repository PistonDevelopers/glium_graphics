#![feature(std_misc, plugin)]

extern crate freetype;
#[macro_use(uniform)]
extern crate glium;
#[plugin]
extern crate glium_macros;
extern crate glutin;
extern crate graphics;
extern crate image;

pub use glyph_cache::GlyphCache;
pub use backend::GliumSurfaceBackEnd;

mod glyph_cache;
mod backend;
mod shader;
