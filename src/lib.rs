extern crate freetype;
#[macro_use(uniform, implement_vertex)]
extern crate glium;
extern crate graphics;
extern crate image;
extern crate shader_version;

pub use window::GliumWindow;
pub use glyph_cache::{ GlyphCache, GlyphTexture };
pub use backend::{ Glium2d, GliumGraphics, DrawTexture };
pub use backend::Glium2d as GliumBackendSystem;
pub use backend::GliumGraphics as GliumSurfaceBackEnd;

mod glyph_cache;
mod backend;
mod shader;
mod window;
