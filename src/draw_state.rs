use glium;
use graphics::draw_state;
use graphics::draw_state::state::BlendChannel;

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
