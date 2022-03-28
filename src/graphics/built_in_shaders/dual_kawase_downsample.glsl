#VERTEX 

#INCLUDE fullscreen_vertex

#FRAGMENT

uniform sampler2D p_texture;
uniform vec2 p_half_pixel;

in vec2 TexCoords;

out vec4 color_out;

void main() {
    vec4 sum = texture(p_texture, TexCoords) * 4.0;
    sum += texture(p_texture, TexCoords - p_half_pixel.xy);
    sum += texture(p_texture, TexCoords + p_half_pixel.xy);
    sum += texture(p_texture, TexCoords + vec2(p_half_pixel.x, -p_half_pixel.y));
    sum += texture(p_texture, TexCoords - vec2(p_half_pixel.x, -p_half_pixel.y));
    color_out = sum / 8.0;
}
