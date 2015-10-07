use glium;
use graphics::draw_state;
use graphics::draw_state::state::BlendChannel;

pub fn convert_draw_state(draw_state: &draw_state::DrawState)
-> glium::draw_parameters::DrawParameters<'static> {
    let (polygon_mode, line_width, point_size, cull) =
        convert_primitive(draw_state.primitive);
    glium::draw_parameters::DrawParameters {
        blend: convert_blend(draw_state.blend),
        color_mask: convert_color_mask(draw_state.color_mask),
        scissor: convert_scissor(draw_state.scissor),
        multisampling: convert_multi_sample(draw_state.multi_sample),
        stencil: convert_stencil(draw_state.stencil, &draw_state.primitive),
        depth: convert_depth(draw_state.depth),
        polygon_mode: polygon_mode,
        line_width: line_width,
        point_size: point_size,
        backface_culling: cull,
        .. Default::default()
    }
}

/// Returns polygon_mode, line_width, point_size, backface_cullingmode
pub fn convert_primitive(p: draw_state::state::Primitive)
-> (glium::draw_parameters::PolygonMode, Option<f32>, Option<f32>,
    glium::draw_parameters::BackfaceCullingMode) {
    use graphics::draw_state::state::{ CullFace, FrontFace, RasterMethod };
    use glium::draw_parameters::{ BackfaceCullingMode, PolygonMode };

    let (front, back) = match p.front_face {
            FrontFace::Clockwise =>
                (BackfaceCullingMode::CullClockWise,
                 BackfaceCullingMode::CullCounterClockWise),
            FrontFace::CounterClockwise =>
                (BackfaceCullingMode::CullCounterClockWise,
                 BackfaceCullingMode::CullClockWise),
        };
    let cull = match p.get_cull_face() {
            CullFace::Nothing => BackfaceCullingMode::CullingDisabled,
            CullFace::Front => front,
            CullFace::Back => back,
    };
    match p.method {
        RasterMethod::Point => (PolygonMode::Point, None, Some(1.0), cull),
        RasterMethod::Line(width) => (PolygonMode::Line, Some(width), None, cull),
        RasterMethod::Fill(_) => (PolygonMode::Fill, None, None, cull),
    }
}

pub fn convert_multi_sample(ms: Option<draw_state::state::MultiSample>)
-> bool {
    match ms {
        None => false,
        Some(_) => true,
    }
}

pub fn convert_scissor(rect: Option<draw_state::target::Rect>)
-> Option<glium::Rect> {
    rect.map(|rect| {
        glium::Rect {
            left: rect.x as u32,
            bottom: rect.y as u32,
            width: rect.w as u32,
            height: rect.h as u32,
        }
    })
}

fn convert_stencil_op(op: draw_state::state::StencilOp)
-> glium::StencilOperation {
    use graphics::draw_state::state::StencilOp;
    use glium::StencilOperation;

    match op {
        StencilOp::Keep => StencilOperation::Keep,
        StencilOp::Zero => StencilOperation::Zero,
        StencilOp::Replace => StencilOperation::Replace,
        StencilOp::IncrementClamp => StencilOperation::Increment,
        StencilOp::IncrementWrap => StencilOperation::IncrementWrap,
        StencilOp::DecrementClamp => StencilOperation::Decrement,
        StencilOp::DecrementWrap => StencilOperation::DecrementWrap,
        StencilOp::Invert => StencilOperation::Invert,
    }
}

fn convert_stencil_test(
    test: draw_state::state::Comparison,
    mask_read: draw_state::target::Stencil
) -> glium::StencilTest {
    use graphics::draw_state::state::Comparison;
    use glium::StencilTest;

    match test {
        Comparison::Never => StencilTest::AlwaysFail,
        Comparison::Less => StencilTest::IfLess { mask: mask_read as u32 },
        Comparison::LessEqual => StencilTest::IfLessOrEqual {
                mask: mask_read as u32
            },
        Comparison::Equal => StencilTest::IfEqual { mask: mask_read as u32 },
        Comparison::GreaterEqual => StencilTest::IfMoreOrEqual {
                mask: mask_read as u32
            },
        Comparison::Greater => StencilTest::IfMore { mask: mask_read as u32 },
        Comparison::NotEqual => StencilTest::IfNotEqual {
                mask: mask_read as u32
            },
        Comparison::Always => StencilTest::AlwaysPass,
    }
}

pub fn convert_stencil(
    stencil: Option<draw_state::state::Stencil>,
    primitive: &draw_state::state::Primitive
)
-> glium::draw_parameters::Stencil {
    use graphics::draw_state::state::FrontFace;

    match stencil {
        None => Default::default(),
        Some(stencil) => {
            let (cc, ccw) = match primitive.front_face {
                FrontFace::Clockwise => (stencil.front, stencil.back),
                FrontFace::CounterClockwise => (stencil.back, stencil.front),
            };
            glium::draw_parameters::Stencil {
                // Clockwise side.
                test_clockwise: convert_stencil_test(cc.fun, cc.mask_read),
                reference_value_clockwise: cc.value as i32,
                write_mask_clockwise: cc.mask_write as u32,
                fail_operation_clockwise: convert_stencil_op(cc.op_fail),
                pass_depth_fail_operation_clockwise:
                    convert_stencil_op(cc.op_depth_fail),
                depth_pass_operation_clockwise:
                    convert_stencil_op(cc.op_pass),
                // Counter clockwise side.
                test_counter_clockwise:
                    convert_stencil_test(ccw.fun, ccw.mask_read),
                reference_value_counter_clockwise: ccw.value as i32,
                write_mask_counter_clockwise: ccw.mask_write as u32,
                fail_operation_counter_clockwise:
                    convert_stencil_op(ccw.op_fail),
                pass_depth_fail_operation_counter_clockwise:
                    convert_stencil_op(ccw.op_depth_fail),
                depth_pass_operation_counter_clockwise:
                    convert_stencil_op(ccw.op_pass),
            }
        }
    }
}

pub fn convert_depth(depth: Option<draw_state::state::Depth>)
-> glium::Depth {
    use graphics::draw_state::state::Comparison;
    use glium::DepthTest;

    match depth {
        None => Default::default(),
        Some(depth) => glium::Depth {
                test: match depth.fun {
                    Comparison::Never => DepthTest::Ignore,
                    Comparison::Less => DepthTest::IfLess,
                    Comparison::LessEqual => DepthTest::IfLessOrEqual,
                    Comparison::Equal => DepthTest::IfEqual,
                    Comparison::GreaterEqual => DepthTest::IfMoreOrEqual,
                    Comparison::Greater => DepthTest::IfMore,
                    Comparison::NotEqual => DepthTest::IfNotEqual,
                    Comparison::Always => DepthTest::Overwrite,
                },
                write: depth.write,
                ..Default::default()
            }
    }
}

fn convert_factor(factor: draw_state::state::Factor)
-> glium::LinearBlendingFactor {
    use graphics::draw_state::state::{ BlendValue, Factor };
    use glium::LinearBlendingFactor;

    match factor {
        Factor::Zero => LinearBlendingFactor::Zero,
        Factor::One => LinearBlendingFactor::One,
        Factor::SourceAlphaSaturated =>
            LinearBlendingFactor::SourceAlphaSaturate,
        Factor::ZeroPlus(BlendValue::SourceColor) =>
            LinearBlendingFactor::SourceColor,
        Factor::ZeroPlus(BlendValue::SourceAlpha) =>
            LinearBlendingFactor::SourceAlpha,
        Factor::ZeroPlus(BlendValue::DestColor) =>
            LinearBlendingFactor::DestinationColor,
        Factor::ZeroPlus(BlendValue::DestAlpha) =>
            LinearBlendingFactor::DestinationAlpha,
        Factor::ZeroPlus(BlendValue::ConstColor) =>
            LinearBlendingFactor::ConstantColor,
        Factor::ZeroPlus(BlendValue::ConstAlpha) =>
            LinearBlendingFactor::ConstantAlpha,
        Factor::OneMinus(BlendValue::SourceColor) =>
            LinearBlendingFactor::SourceColor,
        Factor::OneMinus(BlendValue::SourceAlpha) =>
            LinearBlendingFactor::OneMinusSourceAlpha,
        Factor::OneMinus(BlendValue::DestColor) =>
            LinearBlendingFactor::OneMinusDestinationColor,
        Factor::OneMinus(BlendValue::DestAlpha) =>
            LinearBlendingFactor::OneMinusDestinationAlpha,
        Factor::OneMinus(BlendValue::ConstColor) =>
            LinearBlendingFactor::OneMinusConstantColor,
        Factor::OneMinus(BlendValue::ConstAlpha) =>
            LinearBlendingFactor::OneMinusConstantAlpha,
    }
}

fn convert_blend_channel(blend_channel: BlendChannel)
-> glium::BlendingFunction {
    use graphics::draw_state::state::Equation;
    use glium::BlendingFunction;

    let BlendChannel { equation, source, destination } = blend_channel;
    match equation {
        Equation::Add => {
            BlendingFunction::Addition {
                source: convert_factor(source),
                destination: convert_factor(destination)
            }
        }
        Equation::Sub => {
            BlendingFunction::Subtraction {
                source: convert_factor(source),
                destination: convert_factor(destination)
            }
        }
        Equation::RevSub => {
            BlendingFunction::ReverseSubtraction {
                source: convert_factor(source),
                destination: convert_factor(destination)
            }
        }
        Equation::Min => BlendingFunction::Min,
        Equation::Max => BlendingFunction::Max,
    }
}

pub fn convert_blend(blend: Option<draw_state::state::Blend>) -> glium::Blend {
    use glium::Blend;

    match blend {
        None => Default::default(),
        Some(blend) => {
            let value = blend.value;
            Blend {
                color: convert_blend_channel(blend.color),
                alpha: convert_blend_channel(blend.alpha),
                constant_value: (value[0], value[1], value[2], value[3])
            }
        }
    }
}

pub fn convert_color_mask(mask: draw_state::state::ColorMask)
-> (bool, bool, bool, bool) {
    use graphics::draw_state::state::{ RED, GREEN, BLUE, ALPHA };

    (!(mask & RED).is_empty(),
     !(mask & GREEN).is_empty(),
     !(mask & BLUE).is_empty(),
     !(mask & ALPHA).is_empty())
}
