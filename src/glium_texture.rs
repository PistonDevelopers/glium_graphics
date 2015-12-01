use std::path::Path;

use glium::{ Texture2d };
use glium::texture::{ RawImage2d, TextureCreationError };
use glium::backend::Facade;
use image::{ self, DynamicImage, RgbaImage };
use texture::{ self, ImageSize, TextureSettings, Rgba8Texture };

/// Flip settings.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Flip {
    /// Does not flip.
    None,
    /// Flips image vertically.
    Vertical,
}

/// Wrapper for 2D texture.
pub struct Texture {
    /// The Glium texture.
    pub texture: Texture2d,
}

impl Texture {
    /// Creates a new `Texture`.
    pub fn new(texture: Texture2d) -> Texture {
        Texture { texture: texture }
    }

    /// Returns empty texture.
    pub fn empty<F>(factory: &mut F) -> Result<Self, TextureCreationError>
        where F: Facade
    {
        Rgba8Texture::create(factory, &[0u8; 4], [1, 1], &TextureSettings::new())
    }

    /// Creates a texture from path.
    pub fn from_path<F, P>(
        factory: &mut F,
        path: P,
        flip: Flip,
        settings: &TextureSettings
    ) -> Result<Self, String>
        where F: Facade,
              P: AsRef<Path>
    {
        let img = try!(image::open(path).map_err(|e| e.to_string()));

        let img = match img {
            DynamicImage::ImageRgba8(img) => img,
            img => img.to_rgba()
        };

        let img = if flip == Flip::Vertical {
            image::imageops::flip_vertical(&img)
        } else {
            img
        };

        Texture::from_image(factory, &img, settings).map_err(
            |e| format!("{:?}", e))
    }

    /// Creates a texture from image.
    pub fn from_image<F>(
        factory: &mut F,
        img: &RgbaImage,
        settings: &TextureSettings
    ) -> Result<Self, TextureCreationError>
        where F: Facade
    {
        let (width, height) = img.dimensions();
        Rgba8Texture::create(factory, img, [width, height], settings)
    }

    /// Creates texture from memory alpha.
    pub fn from_memory_alpha<F>(
        factory: &mut F,
        buffer: &[u8],
        width: u32,
        height: u32,
        settings: &TextureSettings
    ) -> Result<Self, TextureCreationError>
        where F: Facade
    {
        if width == 0 || height == 0 {
            return Texture::empty(factory);
        }

        let size = [width, height];
        let buffer = texture::ops::alpha_to_rgba8(buffer, size);
        Rgba8Texture::create(factory, &buffer, size, settings)
    }

    /// Updates texture with an image.
    pub fn update<F>(&mut self, factory: &mut F, img: &RgbaImage)
    -> Result<(), TextureCreationError>
        where F: Facade
    {
        let (width, height) = img.dimensions();
        Rgba8Texture::update(self, factory, img, [width, height])
    }
}

impl ImageSize for Texture {
    fn get_size(&self) -> (u32, u32) {
        let ref tex = self.texture;
        (tex.get_width(), tex.get_height().unwrap())
    }
}

impl<F> Rgba8Texture<F> for Texture
    where F: Facade
{
    type Error = TextureCreationError;

    fn create<S: Into<[u32; 2]>>(
        factory: &mut F,
        memory: &[u8],
        size: S,
        settings: &TextureSettings
    ) -> Result<Self, Self::Error> {
        let size = size.into();
        Ok(Texture {
            texture: try!(Texture2d::new(factory,
                RawImage2d::from_raw_rgba_reversed(memory.to_owned(),
                    (size[0], size[1]))))
        })
    }

    fn update<S: Into<[u32; 2]>>(
        &mut self,
        factory: &mut F,
        memory: &[u8],
        size: S
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }
}
