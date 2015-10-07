use std::default::Default;
use graphics::{ self, DrawState, ImageSize, Graphics };
use glium::{
    Surface, Texture2d, Texture, Program, VertexBuffer, DrawParameters,
};
use glium::index::{ NoIndices, PrimitiveType };
use glium::backend::Facade;
use shader_version::{ Shaders, OpenGL };
use shader_version::glsl::GLSL;

use draw_state;

/// Wrapper for 2D texture.
pub struct DrawTexture {
    /// The Glium texture.
    pub texture: Texture2d,
}

impl DrawTexture {
    /// Creates a new `DrawTexture`.
    pub fn new(texture: Texture2d) -> DrawTexture {
        DrawTexture { texture: texture }
    }
}

impl ImageSize for DrawTexture {
    fn get_size(&self) -> (u32, u32) {
        let ref tex = self.texture;
        (tex.get_width(), tex.get_height().unwrap())
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
    type Texture = DrawTexture;

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

            let (polygon_mode, line_width, point_size, cull) =
                draw_state::convert_primitive(draw_state.primitive);
            self.surface.draw(
                slice,
                &NoIndices(PrimitiveType::TrianglesList),
                &self.system.shader_color,
                &uniform! { color: *color },
                &DrawParameters {
                    blend: draw_state::convert_blend(draw_state.blend),
                    color_mask:
                        draw_state::convert_color_mask(draw_state.color_mask),
                    scissor: draw_state::convert_scissor(draw_state.scissor),
                    multisampling:
                        draw_state::convert_multi_sample(draw_state.multi_sample),
                    stencil: draw_state::convert_stencil(draw_state.stencil,
                        &draw_state.primitive),
                    depth: draw_state::convert_depth(draw_state.depth),
                    polygon_mode: polygon_mode,
                    line_width: line_width,
                    point_size: point_size,
                    backface_culling: cull,
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
        draw_state: &DrawState,
        color: &[f32; 4],
        texture: &DrawTexture,
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
            let (polygon_mode, line_width, point_size, cull) =
                draw_state::convert_primitive(draw_state.primitive);
            self.surface.draw(
                slice,
                &NoIndices(PrimitiveType::TrianglesList),
                &self.system.shader_texture,
                &uniform! {
                    color: *color,
                    s_texture: texture
                },
                &DrawParameters {
                    blend: draw_state::convert_blend(draw_state.blend),
                    color_mask:
                        draw_state::convert_color_mask(draw_state.color_mask),
                    scissor: draw_state::convert_scissor(draw_state.scissor),
                    multisampling:
                        draw_state::convert_multi_sample(draw_state.multi_sample),
                    stencil: draw_state::convert_stencil(draw_state.stencil,
                        &draw_state.primitive),
                    depth: draw_state::convert_depth(draw_state.depth),
                    polygon_mode: polygon_mode,
                    line_width: line_width,
                    point_size: point_size,
                    backface_culling: cull,
                    .. Default::default()
                },
            )
            .ok()
            .expect("failed to draw triangle list");
        })
    }
}
