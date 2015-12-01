use graphics::{ self, DrawState, ImageSize, Graphics };
use glium::{
    Surface, Texture2d, Program, VertexBuffer,
};
use glium::texture::{ TextureCreationError, RawImage2d };
use glium::index::{ NoIndices, PrimitiveType };
use glium::backend::Facade;
use shader_version::{ Shaders, OpenGL };
use shader_version::glsl::GLSL;
use texture::{ self, Rgba8Texture, TextureSettings };
use image::{ self, DynamicImage, RgbaImage };
use std::path::Path;

use draw_state;

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

#[derive(Copy, Clone)]
struct PlainVertex {
    pos: [f32; 2],
}

implement_vertex!(PlainVertex, pos);


#[derive(Copy, Clone)]
struct TexturedVertex {
    pos: [f32; 2],
    uv: [f32; 2],
}

implement_vertex!(TexturedVertex, pos, uv);

/// The resources needed for rendering 2D.
pub struct Glium2d {
    plain_buffer: VertexBuffer<PlainVertex>,
    textured_buffer: VertexBuffer<TexturedVertex>,
    shader_texture: Program,
    shader_color: Program,
}

impl Glium2d {
    /// Creates a new `Glium2d`.
    pub fn new<W>(opengl: OpenGL, window: &W) -> Glium2d where W: Facade {
        use shaders::{ colored, textured };

        let src = |bytes| { unsafe { ::std::str::from_utf8_unchecked(bytes) } };
        let glsl = opengl.to_glsl();

        Glium2d {
            plain_buffer: VertexBuffer::empty_dynamic(window, graphics::BACK_END_MAX_VERTEX_COUNT).unwrap(),
            textured_buffer: VertexBuffer::empty_dynamic(window, graphics::BACK_END_MAX_VERTEX_COUNT).unwrap(),
            shader_texture:
                Program::from_source(window,
                                     Shaders::new().set(GLSL::V1_20, src(textured::VERTEX_GLSL_120))
                                                   .set(GLSL::V1_50, src(textured::VERTEX_GLSL_150_CORE))
                                                   .get(glsl).unwrap(),
                                     Shaders::new().set(GLSL::V1_20, src(textured::FRAGMENT_GLSL_120))
                                                   .set(GLSL::V1_50, src(textured::FRAGMENT_GLSL_150_CORE))
                                                   .get(glsl).unwrap(),
                                     None).ok().expect("failed to initialize textured shader"),
            shader_color:
                Program::from_source(window,
                                     Shaders::new().set(GLSL::V1_20, src(colored::VERTEX_GLSL_120))
                                                   .set(GLSL::V1_50, src(colored::VERTEX_GLSL_150_CORE))
                                                   .get(glsl).unwrap(),
                                     Shaders::new().set(GLSL::V1_20, src(colored::FRAGMENT_GLSL_120))
                                                   .set(GLSL::V1_50, src(colored::FRAGMENT_GLSL_150_CORE))
                                                   .get(glsl).unwrap(),
                                     None).ok().expect("failed to initialize colored shader"),
        }
    }
}


/// Graphics back-end.
pub struct GliumGraphics<'d, 's, S: 's> {
    system: &'d mut Glium2d,
    surface: &'s mut S,
}

impl<'d, 's, S> GliumGraphics<'d, 's, S> {
    /// Creates a new graphics object.
    pub fn new(system: &'d mut Glium2d, surface: &'s mut S)
               -> GliumGraphics<'d, 's, S> {
        GliumGraphics {
            system: system,
            surface: surface,
        }
    }
}

/// Implemented by all graphics back-ends.
impl<'d, 's, S: Surface> Graphics for GliumGraphics<'d, 's, S> {
    type Texture = Texture;

    /// Clears background with a color.
    fn clear_color(&mut self, color: [f32; 4]) {
        let (r, g, b, a) = (color[0], color[1], color[2], color[3]);
        self.surface.clear_color(r, g, b, a);
    }

    fn clear_stencil(&mut self, value: u8) {
        self.surface.clear_stencil(value as i32);
    }


    /// Renders list of 2d triangles.
    fn tri_list<F>(
        &mut self,
        draw_state: &DrawState,
        color: &[f32; 4],
        mut f: F
    )
        where F: FnMut(&mut FnMut(&[f32]))
    {
        f(&mut |vertices: &[f32]| {
            self.system.plain_buffer.invalidate();
            let slice = self.system.plain_buffer.slice(0..vertices.len() / 2).unwrap();

            slice.write({
                &(0 .. vertices.len() / 2)
                    .map(|i| PlainVertex {
                        pos: [vertices[2 * i], vertices[2 * i + 1]]
                    })
                    .collect::<Vec<_>>()
            });

            self.surface.draw(
                slice,
                &NoIndices(PrimitiveType::TrianglesList),
                &self.system.shader_color,
                &uniform! { color: *color },
                &draw_state::convert_draw_state(draw_state),
            )
            .ok()
            .expect("failed to draw triangle list");
        })
    }

    /// Renders list of 2d triangles.
    ///
    /// A texture coordinate is assigned per vertex.
    /// The texture coordinates refers to the current texture.
    fn tri_list_uv<F>(
        &mut self,
        draw_state: &DrawState,
        color: &[f32; 4],
        texture: &Texture,
        mut f: F
    )
        where F: FnMut(&mut FnMut(&[f32], &[f32]))
    {
        use std::cmp::min;

        f(&mut |vertices: &[f32], texture_coords: &[f32]| {
            let len = min(vertices.len(), texture_coords.len()) / 2;

            self.system.textured_buffer.invalidate();
            let slice = self.system.textured_buffer.slice(0..len).unwrap();

            slice.write({
                &(0 .. len)
                    .map(|i| TexturedVertex {
                        pos: [vertices[2 * i], vertices[2 * i + 1]],
                        // FIXME: The `1.0 - ...` is because of a wrong convention
                        uv: [texture_coords[2 * i], 1.0 - texture_coords[2 * i + 1]]
                    })
                    .collect::<Vec<_>>()
            });

            let texture = &texture.texture;
            self.surface.draw(
                slice,
                &NoIndices(PrimitiveType::TrianglesList),
                &self.system.shader_texture,
                &uniform! {
                    color: *color,
                    s_texture: texture
                },
                &draw_state::convert_draw_state(draw_state),
            )
            .ok()
            .expect("failed to draw triangle list");
        })
    }
}
