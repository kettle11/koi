#VERTEX 

#INCLUDE standard_vertex

#FRAGMENT

in vec2 TexCoords;
in vec4 VertexColor;

out vec4 color_out;

// Physically based rendering textures
// These are multipled by the corresponding properties.
uniform sampler2D p_base_color_texture;

void main()
{
    if (TexCoords.x != 0.0 || TexCoords.y != 0.0) {
        color_out = vec4(VertexColor.rgb, texture(p_base_color_texture, TexCoords).r);
    } else {
        color_out = VertexColor;
    }
   // color_out.rgb = pow(color_out.rgb, vec3(1.0/2.2));
}