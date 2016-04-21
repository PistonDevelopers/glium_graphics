use glium;
use graphics::draw_state;

pub fn convert_draw_state(draw_state: &draw_state::DrawState)
-> glium::draw_parameters::DrawParameters<'static> {
    glium::draw_parameters::DrawParameters {
        blend: convert_blend(draw_state.blend),
        scissor: convert_scissor(draw_state.scissor),
        stencil: convert_stencil(draw_state.stencil),
        // polygon_mode: use default (Fill)
        // We don't override Graphics::line, so no need to set line_width.
        // Always draw triangles, whether they are facing clockwise or counterclockwise.
        backface_culling:
            glium::draw_parameters::BackfaceCullingMode::CullingDisabled,
        .. Default::default()
    }
}

pub fn convert_scissor(rect: Option<[u32; 4]>)
-> Option<glium::Rect> {
    rect.map(|rect| {
        // scissor: [x, y, w, h]
        glium::Rect {
            left: rect[0] as u32,
            bottom: rect[1] as u32,
            width: rect[2] as u32,
            height: rect[3] as u32,
        }
    })
}

pub fn convert_stencil(stencil: Option<draw_state::Stencil>)
-> glium::draw_parameters::Stencil {
    use graphics::draw_state::Stencil;
    use glium::{ StencilTest, StencilOperation };

    match stencil {
        None => Default::default(),
        Some(stencil) => {
            let (stencil_test,
                 stencil_test_fail_operation,
                 reference_value) = {
                match stencil {
                    Stencil::Clip(val) => {
                        (StencilTest::AlwaysFail,
                         StencilOperation::Replace,
                         val)
                    }
                    Stencil::Inside(val) => {
                        (StencilTest::IfEqual { mask: 255 },
                         StencilOperation::Keep,
                         val)
                    }
                    Stencil::Outside(val) => {
                        (StencilTest::IfNotEqual { mask: 255 },
                         StencilOperation::Keep,
                         val)
                    }
                }
            };

            // Use triangles for stencil operations, whether they are
            // facing clockwise or counterclockwise.
            glium::draw_parameters::Stencil {
                // Clockwise side.
                test_clockwise: stencil_test,
                reference_value_clockwise: reference_value as i32,
                write_mask_clockwise: 255,
                fail_operation_clockwise: stencil_test_fail_operation,
                pass_depth_fail_operation_clockwise:
                    StencilOperation::Keep,
                depth_pass_operation_clockwise: StencilOperation::Keep,
                // Counter clockwise side.
                test_counter_clockwise: stencil_test,
                reference_value_counter_clockwise: reference_value as i32,
                write_mask_counter_clockwise: 255,
                fail_operation_counter_clockwise:
                    stencil_test_fail_operation,
                pass_depth_fail_operation_counter_clockwise:
                    StencilOperation::Keep,
                depth_pass_operation_counter_clockwise:
                    StencilOperation::Keep,
            }
        }
    }
}

pub fn convert_blend(blend: Option<draw_state::Blend>) -> glium::Blend {
    use graphics::draw_state::Blend;
    use glium::{ BlendingFunction, LinearBlendingFactor };

    match blend {
        None => Default::default(),
        Some(blend) => {
            match blend {
                // What we need is different from glium::Blend::alpha_blending(),
                // so we have to construct this manually.
                Blend::Alpha => {
                    glium::Blend {
                        color: BlendingFunction::Addition {
                            source: LinearBlendingFactor::SourceAlpha,
                            destination: LinearBlendingFactor::OneMinusSourceAlpha
                        },
                        alpha: BlendingFunction::Addition {
                            source: LinearBlendingFactor::One,
                            destination: LinearBlendingFactor::One
                        },
                        constant_value: (0.0, 0.0, 0.0, 0.0)
                    }
                }
                Blend::Add => {
                    glium::Blend {
                        color: BlendingFunction::Addition {
                            source: LinearBlendingFactor::One,
                            destination: LinearBlendingFactor::One
                        },
                        alpha: BlendingFunction::Addition {
                            source: LinearBlendingFactor::One,
                            destination: LinearBlendingFactor::One
                        },
                        constant_value: (0.0, 0.0, 0.0, 0.0)
                    }
                }
                Blend::Multiply => {
                    glium::Blend {
                        color: BlendingFunction::Addition {
                            source: LinearBlendingFactor::DestinationColor,
                            destination: LinearBlendingFactor::Zero
                        },
                        alpha: BlendingFunction::Addition {
                            source: LinearBlendingFactor::DestinationAlpha,
                            destination: LinearBlendingFactor::Zero
                        },
                        constant_value: (0.0, 0.0, 0.0, 0.0)
                    }
                }
                Blend::Invert => {
                    glium::Blend {
                        color: BlendingFunction::Subtraction {
                            source: LinearBlendingFactor::ConstantColor,
                            destination: LinearBlendingFactor::SourceColor
                        },
                        alpha: BlendingFunction::Addition {
                            source: LinearBlendingFactor::Zero,
                            destination: LinearBlendingFactor::One
                        },
                        constant_value: (1.0, 1.0, 1.0, 1.0)
                    }
                }
            }
        }
    }
}
