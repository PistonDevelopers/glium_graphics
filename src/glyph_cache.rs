use std::path::Path;
use std::collections::HashMap;
use graphics::character::{ CharacterCache, Character };
use graphics::types::FontSize;
use glium::Texture2d;
use glium::backend::Facade;
use image::{ Rgba, ImageBuffer };
use freetype::{ self, Face };
use ::back_end::DrawTexture;


fn load_character<F: Facade>(face: &Face, facade: &F, font_size: FontSize,
                             character: char)
    -> Character<DrawTexture>
{
    face.set_pixel_sizes(0, font_size).unwrap();
    face.load_char(character as usize, freetype::face::DEFAULT).unwrap();
    let glyph = face.glyph().get_glyph().unwrap();
    let bitmap_glyph = glyph.to_bitmap(freetype::render_mode::RenderMode::Normal, None).unwrap();
    let bitmap = bitmap_glyph.bitmap();
    let texture =
        if bitmap.width() != 0 { Texture2d::new(
            facade,
            ImageBuffer::<Rgba<u8>, _>::from_raw(
                bitmap.width() as u32, bitmap.rows() as u32,
                bitmap.buffer().iter()
                    .flat_map(|&pix| vec![255, 255, 255, pix].into_iter())
                    .collect::<Vec<_>>()
            ).expect("failed to create glyph texture")
        ) }
        else { Texture2d::empty(facade, 1, 1) };
    let glyph_size_x = glyph.advance_x();
    let glyph_size_y = glyph.advance_y();
    Character {
        offset: [
            bitmap_glyph.left() as f64,
            bitmap_glyph.top() as f64
        ],
        size: [
            (glyph_size_x >> 16) as f64,
            (glyph_size_y >> 16) as f64
        ],
        texture: DrawTexture::new(texture.unwrap()),
    }
}

/// Caches characters for a font.
pub struct GlyphCache<F> {
    face: Face<'static>,
    data: HashMap<FontSize, HashMap<char, Character<DrawTexture>>>,
    facade: F,
}

impl<F> GlyphCache<F> {
     /// Constructor for a GlyphCache.
    pub fn new(font: &Path, facade: F) -> Result<Self, freetype::error::Error> {
        let freetype = try!(freetype::Library::init());
        let face = try!(freetype.new_face(font, 0));
        Ok(GlyphCache {
            face: face,
            data: HashMap::new(),
            facade: facade,
        })
    }
}

impl<F: Facade> CharacterCache for GlyphCache<F> {
    type Texture = DrawTexture;

    fn character<'a>(&'a mut self, font_size: FontSize, character: char)
        -> &'a Character<DrawTexture>
    {
        use std::collections::hash_map::Entry::{Vacant, Occupied};
        let size_cache: &'a mut HashMap<char, _> = match self.data.entry(font_size) {
            Vacant(entry) => entry.insert(HashMap::new()),
            Occupied(entry) => entry.into_mut(),
        };
        match size_cache.entry(character) {
            Vacant(entry) => entry.insert(load_character(&self.face, &self.facade, font_size, character)),
            Occupied(entry) => entry.into_mut(),
        }
    }
}
