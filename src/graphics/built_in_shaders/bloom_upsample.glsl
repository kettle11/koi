#VERTEX 

#INCLUDE fullscreen_vertex

#FRAGMENT

uniform sampler2D p_texture;
uniform sampler2D p_corresponding_downsample_texture;

uniform vec2 p_half_pixel;

in vec2 TexCoords;

out vec4 color_out;

vec4 dual_kawase() {
    vec4 sum = texture(p_texture, TexCoords + vec2(-p_half_pixel.x * 2.0, 0.0));
    sum += texture(p_texture, TexCoords + vec2(-p_half_pixel.x, p_half_pixel.y)) * 2.0;
    sum += texture(p_texture, TexCoords + vec2(0.0, p_half_pixel.y * 2.0));
    sum += texture(p_texture, TexCoords + vec2(p_half_pixel.x, p_half_pixel.y)) * 2.0;
    sum += texture(p_texture, TexCoords + vec2(p_half_pixel.x * 2.0, 0.0));
    sum += texture(p_texture, TexCoords + vec2(p_half_pixel.x, -p_half_pixel.y)) * 2.0;
    sum += texture(p_texture, TexCoords + vec2(0.0, -p_half_pixel.y * 2.0));
    sum += texture(p_texture, TexCoords + vec2(-p_half_pixel.x, -p_half_pixel.y)) * 2.0;
    return sum / 12.0;
}

void main() {
    vec4 p = texture(p_corresponding_downsample_texture, TexCoords);
    color_out = mix(dual_kawase(), p, 0.1);
}
