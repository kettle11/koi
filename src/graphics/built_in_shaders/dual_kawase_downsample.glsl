#VERTEX 

#INCLUDE fullscreen_vertex

#FRAGMENT

uniform sampler2D p_texture;
uniform vec2 p_half_pixel;
uniform vec2 p_texture_coordinate_scale;

in vec2 TexCoords;

out vec4 color_out;


void main() {
    vec4 sum = texture(p_texture, TexCoords) * 4.0;
    sum += texture(p_texture, min(TexCoords - p_half_pixel.xy,p_texture_coordinate_scale));
    sum += texture(p_texture, min(TexCoords + p_half_pixel.xy,p_texture_coordinate_scale));
    sum += texture(p_texture, min(TexCoords + vec2(p_half_pixel.x, -p_half_pixel.y),p_texture_coordinate_scale));
    sum += texture(p_texture, min(TexCoords - vec2(p_half_pixel.x, -p_half_pixel.y),p_texture_coordinate_scale));
    color_out = sum / 8.0;
}
