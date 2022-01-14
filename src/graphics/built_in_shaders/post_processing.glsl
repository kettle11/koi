#VERTEX 

#INCLUDE standard_vertex

#FRAGMENT

uniform sampler2D p_base_color_texture;

out vec4 color_out;

void main() {
    vec4 color = texture(p_base_color_texture, TexCoords);
    // To-do: Dither, gamma correct, tonemap, bloom?
    color_out = color;
}