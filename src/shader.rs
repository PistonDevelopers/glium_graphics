pub const VS_COLORED_120: &'static str = "
#version 120
uniform vec4 color;

attribute vec2 position;

void main()
{
    gl_Position = vec4(position, 0.0, 1.0);
}
";

pub const VS_COLORED_150: &'static str = "
#version 150 core
uniform vec4 color;

in vec2 position;

void main()
{
    gl_Position = vec4(position, 0.0, 1.0);
}
";


pub const FS_COLORED_120: &'static str = "
#version 120
uniform vec4 color;

void main()
{
    gl_FragColor = color;
}
";

pub const FS_COLORED_150: &'static str = "
#version 150 core
uniform vec4 color;

out vec4 o_Color;

void main()
{
    o_Color = color;
}
";


pub const VS_TEXTURED_120: &'static str = "
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

pub const VS_TEXTURED_150: &'static str = "
#version 150 core
uniform sampler2D s_texture;
uniform vec4 color;

in vec2 position;
in vec2 texcoord;

out vec2 v_texcoord;

void main()
{
    v_texcoord = texcoord;
    gl_Position = vec4(position, 0.0, 1.0);
}
";


pub const FS_TEXTURED_120: &'static str = "
#version 120
uniform vec4 color;
uniform sampler2D s_texture;

varying vec2 v_texcoord;

void main()
{
    gl_FragColor = texture2D(s_texture, v_texcoord) * color;
}
";

pub const FS_TEXTURED_150: &'static str = "
#version 150 core
uniform sampler2D s_texture;
uniform vec4 color;

out vec4 o_Color;

in vec2 v_texcoord;

void main()
{
    o_Color = texture(s_texture, v_texcoord) * color;
}
";
