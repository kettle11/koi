#VERTEX 

in vec3 a_position;
in vec2 a_texture_coordinate;

uniform mat4 p_model;
uniform mat4 p_view;
uniform mat4 p_projection;

out vec2 TexCoords;
out vec3 WorldPosition;

void main()
{
    WorldPosition = vec3(p_model * vec4(a_position, 1.0));
    TexCoords = a_texture_coordinate;
    gl_Position = p_projection * p_view * p_model * vec4(a_position, 1.0);
}

#FRAGMENT

in vec2 TexCoords;
in vec3 WorldPosition;  

out vec4 color_out;

// Physically based rendering properties.
// These adjust the entire model.
uniform vec4 p_base_color;

// Physically based rendering textures
// These are multipled by the corresponding properties.
uniform sampler2D p_base_color_texture;

uniform float p_dither_scale;

// ----------------------------------------------------------------------------

// Portal 2 Screenspace dithering (modified for VR):
// http://media.steampowered.com/apps/valve/2015/Alex_Vlachos_Advanced_VR_Rendering_GDC2015.pdf
vec3 ScreenSpaceDither( vec2 vScreenPos )
{
    vec3 vDither = vec3(dot( vec2( 171.0, 231.0 ), vScreenPos + 0.0 )); // the 0.0 should be time
    vDither.rgb = fract( vDither.rgb / vec3( 103.0, 71.0, 97.0 ) ) - vec3( 0.5, 0.5, 0.5 );
    return ( vDither.rgb / 255.0 ) * 0.375;
}

void main()
{
    vec4 base_color = (p_base_color * texture(p_base_color_texture, TexCoords));
  
    vec3 dither = ScreenSpaceDither(gl_FragCoord.xy) * p_dither_scale;
    vec3 color = pow(base_color.rgb, vec3(1.0/2.2)) + dither; 
        
    color_out = texture(p_base_color_texture, TexCoords);
}