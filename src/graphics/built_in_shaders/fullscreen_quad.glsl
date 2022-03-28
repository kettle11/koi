#VERTEX 

#INCLUDE fullscreen_vertex

#FRAGMENT

in vec2 TexCoords;

uniform sampler2D p_texture;

out vec4 color_out;

void main()
{
    color_out = texture(p_texture, TexCoords);
}