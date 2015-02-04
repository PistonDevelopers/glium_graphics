pub static VS_COLORED_120: &'static str = "
#version 120
uniform vec4 color;

attribute vec4 pos;

void main()
{
    gl_Position = pos;
}
";

pub static FS_COLORED_120: &'static str = "
#version 120
uniform vec4 color;

void main()
{
    gl_FragColor = color;
}
";

pub static VS_TEXTURED_120: &'static str = "
#version 120
uniform vec4 color;

attribute vec4 pos;
attribute vec2 uv;

uniform sampler2D s_texture;

varying vec2 v_uv;

void main()
{
    v_uv = uv;
    gl_Position = pos;
}
";

pub static FS_TEXTURED_120: &'static str = "
#version 120
uniform vec4 color;
uniform sampler2D s_texture;

varying vec2 v_uv;

void main()
{
    gl_FragColor = texture2D(s_texture, v_uv) * color;
}
";
