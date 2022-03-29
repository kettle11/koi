#VERTEX 

#INCLUDE fullscreen_vertex

#FRAGMENT

in vec2 TexCoords;

uniform sampler2D p_texture;
uniform sampler2D p_blurred_texture;

out vec4 color_out;

// Portal 2 Screenspace dithering (modified for VR):
// http://media.steampowered.com/apps/valve/2015/Alex_Vlachos_Advanced_VR_Rendering_GDC2015.pdf
vec3 ScreenSpaceDither( vec2 vScreenPos )
{
    vec3 vDither = vec3(dot( vec2( 171.0, 231.0 ), vScreenPos + 0.0 )); // the 0.0 should be time
    vDither.rgb = fract( vDither.rgb / vec3( 103.0, 71.0, 97.0 ) ) - vec3( 0.5, 0.5, 0.5 );
    return ( vDither.rgb / 255.0 ) * 0.375;
}

const float DITHER_SCALE = 4.0;

void main()
{
    color_out = texture(p_texture, TexCoords);

    // Bloom 
    color_out = mix(color_out, texture(p_blurred_texture, TexCoords), 0.1);

    // Reinhard tonehamp
   // color_out.rgb = color_out.rgb / (color_out.rgb + vec3(1.0));
    
    color_out.rgb = pow(color_out.rgb, vec3(1.0/2.2)); 
    color_out.rgb += ScreenSpaceDither(gl_FragCoord.xy) * DITHER_SCALE;
    color_out.a = 1.0;
}