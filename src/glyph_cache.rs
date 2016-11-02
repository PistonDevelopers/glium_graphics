use std::path::Path;
use std::collections::hash_map::{ HashMap, Entry };
use std::io;
use graphics::character::{ CharacterCache, Character };
use graphics::types::{ FontSize, Scalar };
use glium::backend::Facade;
use rusttype::{
    point, Font, FontCollection, GlyphId, Point, Rect, Scale
};
use Texture;
use TextureSettings;

/// An enum to represent various possible run-time errors that may occur.
#[derive(Debug)]
pub enum Error {
    /// An io error happened when reading font files.
    IoError(io::Error),
    /// No font was found in the file.
    NoFont,
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Error::IoError(io_error)
    }
}

/// Caches characters for a font.
pub struct GlyphCache<F> {
    font: Font<'static>,
    data: HashMap<(FontSize, char), ([Scalar; 2], [Scalar; 2], Texture)>,
    facade: F,
}

impl<F> GlyphCache<F> where F: Facade {
     /// Constructor for a GlyphCache.
    pub fn new<P>(font_path: P, facade: F) -> Result<Self, Error>
        where P: AsRef<Path>
    {
        use std::io::Read;
        use std::fs::File;

        let mut file = try!(File::open(font_path));
        let mut file_buffer = Vec::new();
        try!(file.read_to_end(&mut file_buffer));

        let collection = FontCollection::from_bytes(file_buffer);
        let font = match collection.into_font() {
            Some(font) => font,
            None => return Err(Error::NoFont),
        };

        Ok(GlyphCache {
            font: font,
            data: HashMap::new(),
            facade: facade,
        })
    }

    /*
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
    */
}

impl<F: Facade> CharacterCache for GlyphCache<F> {
    type Texture = Texture;

    /*
    fn character<'a>(&'a mut self, size: FontSize, ch: char) -> Character<'a, Texture> {
        let &(offset, size, ref texture) = self.get(size, ch);
        return Character {
            offset: offset,
            size: size,
            texture: texture
        }
    }
    */

    fn character<'a>(
        &'a mut self,
        size: FontSize,
        ch: char
    ) -> Character<'a, Self::Texture> {
        let size = ((size as f32) * 1.333).round() as u32 ; // convert points to pixels

        match self.data.entry((size, ch)) {
            //returning `into_mut()' to get reference with 'a lifetime
            Entry::Occupied(v) => {
                let &mut (offset, size, ref texture) = v.into_mut();
                Character {
                    offset: offset,
                    size: size,
                    texture: texture
                }
            }
            Entry::Vacant(v) => {
                let glyph = self.font.glyph(ch).unwrap(); // this is only None for invalid GlyphIds, but char is converted to a Codepoint which must result in a glyph.
                let scale = Scale::uniform(size as f32);
                let mut glyph = glyph.scaled(scale);

                // some fonts do not contain glyph zero as fallback, instead try U+FFFD.
                if glyph.id() == GlyphId(0) && glyph.shape().is_none() {
                    glyph = self.font.glyph('\u{FFFD}').unwrap().scaled(scale);
                }

                let h_metrics = glyph.h_metrics();
                let bounding_box = glyph.exact_bounding_box()
                    .unwrap_or(Rect {
                        min: Point{x: 0.0, y: 0.0},
                        max: Point{x: 0.0, y: 0.0}
                    });
                let glyph = glyph.positioned(point(0.0, 0.0));
                let pixel_bounding_box = glyph.pixel_bounding_box()
                    .unwrap_or(Rect {
                        min: Point{x: 0, y: 0},
                        max: Point{x: 0, y: 0}
                    });
                let pixel_bb_width = pixel_bounding_box.width();
                let pixel_bb_height = pixel_bounding_box.height();

                let mut image_buffer = Vec::<u8>::new();
                image_buffer.resize((pixel_bb_width * pixel_bb_height) as usize, 0);
                glyph.draw(|x, y, v| {
                   let pos = (x + y * (pixel_bb_width as u32)) as usize;
                   image_buffer[pos] = (255.0 * v) as u8;
                });

                let &mut (offset, size, ref texture) = v.insert((
                    [
                        bounding_box.min.x as Scalar,
                        -pixel_bounding_box.min.y as Scalar,
                    ],
                    [
                        h_metrics.advance_width as Scalar,
                        0 as Scalar,
                    ],
                    {
                        if pixel_bb_width == 0 || pixel_bb_height == 0 {
                            Texture::empty(&mut self.facade)
                                    .unwrap()
                        } else {
                            Texture::from_memory_alpha(
                                &mut self.facade,
                                &image_buffer,
                                pixel_bb_width as u32,
                                pixel_bb_height as u32,
                                &TextureSettings::new()
                            ).unwrap()
                        }
                    },
                ));
                Character {
                    offset: offset,
                    size: size,
                    texture: texture
                }
            }
        }
    }
}
