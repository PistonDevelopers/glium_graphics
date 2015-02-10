pub static VS_COLORED_120: &'static str = "
#version 120
uniform vec4 color;

attribute vec2 position;

void main()
{
    gl_Position = vec4(position, 0.0, 1.0);
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

attribute vec2 position;
attribute vec2 texcoord;

uniform sampler2D s_texture;

varying vec2 v_texcoord;

void main()
{
    v_texcoord = texcoord;
    gl_Position = vec4(position, 0.0, 1.0);
}
";

pub static FS_TEXTURED_120: &'static str = "
#version 120
uniform vec4 color;
uniform sampler2D s_texture;

varying vec2 v_texcoord;

void main()
{
    gl_FragColor = texture2D(s_texture, v_texcoord) * color;
}
";
