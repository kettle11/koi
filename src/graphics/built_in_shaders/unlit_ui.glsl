#VERTEX 

#INCLUDE standard_vertex

#FRAGMENT

in vec2 TexCoords;
in vec4 VertexColor;

out vec4 color_out;

uniform sampler2D p_base_color_texture;

void main()
{
    if (TexCoords.x != 0.0 || TexCoords.y != 0.0) {
        vec4 v = texture(p_base_color_texture, TexCoords);
        color_out = vec4(VertexColor.rgb * v.rgb * v.a, v.a);
    } else {
        color_out = VertexColor;
    }
    color_out.rgb = pow(color_out.rgb, vec3(1.0/2.2));
}