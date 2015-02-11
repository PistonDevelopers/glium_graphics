use std::sync::Arc;
use std::default::Default;
use graphics::{self, ImageSize, BackEnd};
use glium::{Display, Surface, Texture2d, Texture, Program, VertexBuffer,
            DrawParameters, BlendingFunction, LinearBlendingFactor};
use glium::index::{NoIndices, PrimitiveType};
use shader;


#[derive(Clone)]
pub struct DrawTexture {
    texture: Arc<Texture2d>,
}

impl DrawTexture {
    pub fn new(texture: Texture2d) -> DrawTexture {
        DrawTexture { texture: Arc::new(texture) }
    }
}

impl ImageSize for DrawTexture {
    fn get_size(&self) -> (u32, u32) {
        let ref tex = self.texture;
        (tex.get_width(), tex.get_height().unwrap())
    }
}


#[vertex_format]
#[derive(Copy, Clone)]
struct PlainVertex {
    position: [f32; 2],
}


#[vertex_format]
#[derive(Copy, Clone)]
struct TexturedVertex {
    position: [f32; 2],
    texcoord: [f32; 2],
}


pub struct GliumBackendSystem {
    plain_buffer: VertexBuffer<PlainVertex>,
    textured_buffer: VertexBuffer<TexturedVertex>,
    shader_texture: Program,
    shader_color: Program,
}

impl GliumBackendSystem {
    pub fn new(display: &Display) -> GliumBackendSystem {
        // FIXME: create empty buffers when glium supports them
        let plain_data = ::std::iter::repeat(PlainVertex { position: [0.0, 0.0] })
                                .take(graphics::BACK_END_MAX_VERTEX_COUNT).collect();
        let textured_data = ::std::iter::repeat(TexturedVertex { position: [0.0, 0.0], texcoord: [0.0, 0.0] })
                                .take(graphics::BACK_END_MAX_VERTEX_COUNT).collect();

        GliumBackendSystem {
            plain_buffer: VertexBuffer::new(display, plain_data),
            textured_buffer: VertexBuffer::new(display, textured_data),
            shader_texture: Program::from_source(display,
                            shader::VS_TEXTURED_120, shader::FS_TEXTURED_120, None)
                            .ok().expect("failed to initialize textured shader"),
            shader_color: Program::from_source(&display,
                            shader::VS_COLORED_120, shader::FS_COLORED_120, None)
                            .ok().expect("failed to initialize colored shader"),
        }
    }
}


pub struct GliumSurfaceBackEnd<'d, 's, S: 's> {
    system: &'d mut GliumBackendSystem,
    surface: &'s mut S,
    draw_texture: Option<DrawTexture>,
    draw_color: Option<[f32; 4]>,
}

impl<'d, 's, S> GliumSurfaceBackEnd<'d, 's, S> {
    pub fn new(system: &'d mut GliumBackendSystem, surface: &'s mut S)
               -> GliumSurfaceBackEnd<'d, 's, S> {
        GliumSurfaceBackEnd {
            system: system,
            surface: surface,
            draw_texture: None,
            draw_color: None,
        }
    }
}

/// Implemented by all graphics back-ends.
impl<'d, 's, S: Surface> BackEnd for GliumSurfaceBackEnd<'d, 's, S> {
    type Texture = DrawTexture;

    /// Clears background with a color.
    fn clear(&mut self, color: [f32; 4]) {
        let [r, g, b, a] = color;
        self.surface.clear_color(r, g, b, a);
    }

    /// Sets the texture.
    fn enable_texture(&mut self, texture: &DrawTexture) {
        self.draw_texture = Some(texture.clone());
    }

    /// Disables texture.
    fn disable_texture(&mut self) {
        self.draw_texture = None;
    }

    /// Sets the current color.
    fn color(&mut self, color: [f32; 4]) {
        self.draw_color = Some(color);
    }

    /// Renders list of 2d triangles.
    fn tri_list(&mut self, vertices: &[f32]) {
        let slice = self.system.plain_buffer.slice(0, vertices.len() / 2).unwrap();

        slice.write({
            (0..vertices.len() / 2)
                .map(|i| PlainVertex {
                    position: [vertices[2 * i], vertices[2 * i + 1]],
                })
                .collect()
        });

        self.surface.draw(
            slice,
            &NoIndices(PrimitiveType::TrianglesList),
            &self.system.shader_color,
            &uniform! { color: self.draw_color.unwrap_or([1., 1., 1., 1.]) },
            &DrawParameters {
                blending_function: Some(BlendingFunction::Addition {
                    source: LinearBlendingFactor::SourceAlpha,
                    destination: LinearBlendingFactor::OneMinusSourceAlpha,
                }),
                .. Default::default()
            },
        )
        .ok()
        .expect("failed to draw triangle list");
    }

    /// Renders list of 2d triangles.
    ///
    /// A texture coordinate is assigned per vertex.
    /// The texture coordinates refers to the current texture.
    fn tri_list_uv(&mut self, vertices: &[f32], texture_coords: &[f32]) {
        use std::cmp::min;

        let len = min(vertices.len(), texture_coords.len()) / 2;
        let slice = self.system.textured_buffer.slice(0, len).unwrap();

        slice.write({
            (0..len)
                .map(|i| TexturedVertex {
                    position: [vertices[2 * i], vertices[2 * i + 1]],
                    // FIXME: The `1.0 - ...` is because of a wrong convention
                    texcoord: [texture_coords[2 * i], 1.0 - texture_coords[2 * i + 1]],
                })
                .collect()
        });

        let texture = &*(self.draw_texture.as_ref().unwrap().texture);
        self.surface.draw(
            slice,
            &NoIndices(PrimitiveType::TrianglesList),
            &self.system.shader_texture,
            &uniform! {
                color: self.draw_color.unwrap_or([1., 1., 1., 1.]),
                s_texture: texture
            },
            &DrawParameters {
                blending_function: Some(BlendingFunction::Addition {
                    source: LinearBlendingFactor::SourceAlpha,
                    destination: LinearBlendingFactor::OneMinusSourceAlpha,
                }),
                .. Default::default()
            },
        )
        .ok()
        .expect("failed to draw triangle list");
    }
}
