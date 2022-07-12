#VERTEX 

#INCLUDE standard_vertex

#FRAGMENT

in vec2 TexCoords;
in vec3 WorldPosition;  
in vec4 VertexColor;
in vec3 Normal;

uniform vec2 p_texture_coordinate_offset;
uniform vec2 p_texture_coordinate_scale;

out vec4 color_out;

// Physically based rendering properties.
// These adjust the entire model.
uniform vec4 p_base_color;

// Physically based rendering textures
// These are multipled by the corresponding properties.
uniform sampler2D p_base_color_texture;

void main()
{
  vec4 base_color = (VertexColor * p_base_color * texture(p_base_color_texture, TexCoords * p_texture_coordinate_scale + p_texture_coordinate_offset));
  color_out = base_color;
}