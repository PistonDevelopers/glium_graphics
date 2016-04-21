use std::path::Path;
use std::collections::HashMap;
use graphics::character::{ CharacterCache, Character };
use graphics::types::{ FontSize, Scalar };
use glium::texture::srgb_texture2d::SrgbTexture2d;
use glium::backend::Facade;
use glium::texture::RawImage2d;
use image::{ Rgba, ImageBuffer };
use freetype::{ self, Face };
use Texture;

/// Caches characters for a font.
pub struct GlyphCache<F> {
    face: Face<'static>,
    data: HashMap<(FontSize, char), ([Scalar; 2], [Scalar; 2], Texture)>,
    facade: F,
}

impl<F> GlyphCache<F> where F: Facade {
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

    /// Get a `Character` from cache, or load it if not there.
    fn get(&mut self, font_size: FontSize, character: char)
        -> &([Scalar; 2], [Scalar; 2], Texture)
    {
        // Create a `Character` from a given `FontSize` and `char`.
        fn create_character<F: Facade>(
            facade: &F,
            face: &freetype::Face,
            font_size: FontSize,
            character: char
        ) -> ([Scalar; 2], [Scalar; 2], Texture) {
            face.set_pixel_sizes(0, font_size).unwrap();
            face.load_char(character as usize, freetype::face::DEFAULT).unwrap();
            let glyph = face.glyph().get_glyph().unwrap();
            let bitmap_glyph = glyph.to_bitmap(freetype::render_mode::RenderMode::Normal, None).unwrap();
            let bitmap = bitmap_glyph.bitmap();
            let texture =
                if bitmap.width() != 0 {
                    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(
                        bitmap.width() as u32, bitmap.rows() as u32,
                        bitmap.buffer().iter()
                            .flat_map(|&pix| vec![255, 255, 255, pix].into_iter())
                            .collect::<Vec<_>>()
                        ).expect("failed to create glyph texture");
                    let image_dimensions = image.dimensions();
                    SrgbTexture2d::new(
                        facade,
                        RawImage2d::from_raw_rgba_reversed(
                            image.into_raw(), image_dimensions
                        )
                    )
                } else { SrgbTexture2d::empty(facade, 1, 1) };
            let glyph_size_x = glyph.advance_x();
            let glyph_size_y = glyph.advance_y();
            (
                [bitmap_glyph.left() as f64, bitmap_glyph.top() as f64],
                [(glyph_size_x >> 16) as f64, (glyph_size_y >> 16) as f64],
                Texture::new(texture.unwrap())
            )
        }

        let face = &self.face; // necessary to borrow-check
        let facade = &self.facade;

        self.data.entry((font_size, character))
                 .or_insert_with(|| create_character(facade, face, font_size, character) )
    }
}

impl<F: Facade> CharacterCache for GlyphCache<F> {
    type Texture = Texture;

    fn character<'a>(&'a mut self, size: FontSize, ch: char) -> Character<'a, Texture> {
        let &(offset, size, ref texture) = self.get(size, ch);
        return Character {
            offset: offset,
            size: size,
            texture: texture
        }
    }
}
