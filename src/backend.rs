use std::default::Default;
use graphics::{ DrawState, Graphics, BACK_END_MAX_VERTEX_COUNT };
use glium_lib::{
    Display, Surface, Program, VertexBuffer,
    DrawParameters, BlendingFunction, LinearBlendingFactor
};
use glium_lib::index::{ NoIndices, PrimitiveType };
use shader_version::{ Shaders, OpenGL };
use shader_version::glsl::GLSL;

use { shader, GliumTexture };


#[derive(Copy, Clone)]
struct PlainVertex {
    position: [f32; 2],
}

implement_vertex!(PlainVertex, position);


#[derive(Copy, Clone)]
struct TexturedVertex {
    position: [f32; 2],
    texcoord: [f32; 2],
}

implement_vertex!(TexturedVertex, position, texcoord);


pub struct Glium2d {
    next_plain_buffer: u32,
    plain_buffer1: VertexBuffer<PlainVertex>,
    plain_buffer2: VertexBuffer<PlainVertex>,
    next_textured_buffer: u32,
    textured_buffer1: VertexBuffer<TexturedVertex>,
    textured_buffer2: VertexBuffer<TexturedVertex>,
    shader_texture: Program,
    shader_color: Program,
}

impl Glium2d {
    pub fn new(opengl: OpenGL, display: &Display) -> Glium2d {
        // FIXME: create empty buffers when glium supports them
        let plain_data = vec![PlainVertex { position: [0.0, 0.0] }; BACK_END_MAX_VERTEX_COUNT];
        let textured_data = vec![TexturedVertex { position: [0.0, 0.0], texcoord: [0.0, 0.0] };
                                 BACK_END_MAX_VERTEX_COUNT];

        Glium2d {
            next_plain_buffer: 0,
            plain_buffer1: VertexBuffer::new(display, plain_data.clone()),
            plain_buffer2: VertexBuffer::new(display, plain_data),
            next_textured_buffer: 0,
            textured_buffer1: VertexBuffer::new(display, textured_data.clone()),
            textured_buffer2: VertexBuffer::new(display, textured_data),
            shader_texture:
                Program::from_source(display,
                                     Shaders::new().set(GLSL::_1_20, shader::VS_TEXTURED_120)
                                                   .set(GLSL::_1_50, shader::VS_TEXTURED_150)
                                                   .get(opengl.to_GLSL()).unwrap(),
                                     Shaders::new().set(GLSL::_1_20, shader::FS_TEXTURED_120)
                                                   .set(GLSL::_1_50, shader::FS_TEXTURED_150)
                                                   .get(opengl.to_GLSL()).unwrap(),
                                     None).ok().expect("failed to initialize textured shader"),
            shader_color:
                Program::from_source(&display,
                                     Shaders::new().set(GLSL::_1_20, shader::VS_COLORED_120)
                                                   .set(GLSL::_1_50, shader::VS_COLORED_150)
                                                   .get(opengl.to_GLSL()).unwrap(),
                                     Shaders::new().set(GLSL::_1_20, shader::FS_COLORED_120)
                                                   .set(GLSL::_1_50, shader::FS_COLORED_150)
                                                   .get(opengl.to_GLSL()).unwrap(),
                                     None).ok().expect("failed to initialize colored shader"),
        }
    }
}


pub struct GliumGraphics<'d, 's, S: 's> {
    system: &'d mut Glium2d,
    surface: &'s mut S,
}

impl<'d, 's, S> GliumGraphics<'d, 's, S> {
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
    type Texture = GliumTexture;

    /// Clears background with a color.
    fn clear(&mut self, color: [f32; 4]) {
        let [r, g, b, a] = color;
        self.surface.clear_color(r, g, b, a);
    }

    /// Renders list of 2d triangles.
    fn tri_list<F>(
        &mut self,
        _draw_state: &DrawState,
        color: &[f32; 4],
        mut f: F
    )
        where F: FnMut(&mut FnMut(&[f32]))
    {
        f(&mut |vertices: &[f32]| {
            let slice = match self.system.next_plain_buffer {
                0 => {
                    self.system.next_plain_buffer = 1;
                    self.system.plain_buffer1.slice(0, vertices.len() / 2).unwrap()
                },
                1 => {
                    self.system.next_plain_buffer = 0;
                    self.system.plain_buffer2.slice(0, vertices.len() / 2).unwrap()
                },
                _ => unreachable!()
            };

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
                &uniform! { color: *color },
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
        })
    }

    /// Renders list of 2d triangles.
    ///
    /// A texture coordinate is assigned per vertex.
    /// The texture coordinates refers to the current texture.
    fn tri_list_uv<F>(
        &mut self,
        _draw_state: &DrawState,
        color: &[f32; 4],
        texture: &GliumTexture,
        mut f: F
    )
        where F: FnMut(&mut FnMut(&[f32], &[f32]))
    {
        use std::cmp::min;

        f(&mut |vertices: &[f32], texture_coords: &[f32]| {
            let len = min(vertices.len(), texture_coords.len()) / 2;

            let slice = match self.system.next_textured_buffer {
                0 => {
                    self.system.next_textured_buffer = 1;
                    self.system.textured_buffer1.slice(0, len).unwrap()
                },
                1 => {
                    self.system.next_textured_buffer = 0;
                    self.system.textured_buffer2.slice(0, len).unwrap()
                },
                _ => unreachable!()
            };

            slice.write({
                (0..len)
                    .map(|i| TexturedVertex {
                        position: [vertices[2 * i], vertices[2 * i + 1]],
                        // FIXME: The `1.0 - ...` is because of a wrong convention
                        texcoord: [texture_coords[2 * i], 1.0 - texture_coords[2 * i + 1]],
                    })
                    .collect()
            });

            self.surface.draw(
                slice,
                &NoIndices(PrimitiveType::TrianglesList),
                &self.system.shader_texture,
                &uniform! {
                    color: *color,
                    s_texture: texture.get_texture2d()
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
        })
    }
}
