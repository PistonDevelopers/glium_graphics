use std::collections::HashMap;
use graphics::character::{ CharacterCache, Character };
use graphics::internal::FontSize;
use graphics::ImageSize;
use glium_lib::{ Texture2d, Texture, Display };
use freetype::{ self, Face };


pub struct GlyphTexture(pub Texture2d);

impl ImageSize for GlyphTexture {
    fn get_size(&self) -> (u32, u32) {
        let GlyphTexture(ref tex) = *self;
        (tex.get_width(), tex.get_height().unwrap())
    }
}


fn load_character(face: &Face, display: &Display, font_size: FontSize, character: char)
    -> Character<GlyphTexture>
{
    face.set_pixel_sizes(0, font_size).unwrap();
    face.load_char(character as usize, freetype::face::DEFAULT).unwrap();
    let glyph = face.glyph().get_glyph().unwrap();
    let bitmap_glyph = glyph.to_bitmap(freetype::render_mode::RenderMode::Normal, None).unwrap();
    let bitmap = bitmap_glyph.bitmap();
    let texture = Texture2d::new::<Vec<Vec<(u8, u8, u8, u8)>>>(
        display,
        bitmap.buffer().chunks(bitmap.width() as usize)
              .map(|row| row.iter().map(|&p| (255, 255, 255, p)).collect()).rev().collect()
    );
    let glyph_size = glyph.advance();
    Character {
        offset: [
            bitmap_glyph.left() as f64,
            bitmap_glyph.top() as f64
        ],
        size: [
            (glyph_size.x >> 16) as f64,
            (glyph_size.y >> 16) as f64
        ],
        texture: GlyphTexture(texture),
    }
}


pub struct GlyphCache {
    face: Face,
    data: HashMap<FontSize, HashMap<char, Character<GlyphTexture>>>,
    display: Display,
}


impl CharacterCache for GlyphCache {
    type Texture = GlyphTexture;

    fn character<'a>(&'a mut self, font_size: FontSize, character: char)
        -> &'a Character<GlyphTexture>
    {
        use std::collections::hash_map::Entry::{Vacant, Occupied};
        let size_cache: &'a mut HashMap<char, _> = match self.data.entry(font_size) {
            Vacant(entry) => entry.insert(HashMap::new()),
            Occupied(entry) => entry.into_mut(),
        };
        match size_cache.entry(character) {
            Vacant(entry) => entry.insert(load_character(&self.face, &self.display, font_size, character)),
            Occupied(entry) => entry.into_mut(),
        }
    }
}
