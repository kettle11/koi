#VERTEX

in vec3 a_position;

uniform mat4 p_views[NUM_VIEWS];
uniform mat4 p_projections[NUM_VIEWS];

out vec3 local_position;

void main()
{
    mat4 rotView = mat4(mat3(p_views[0])); // remove translation from the view matrix
    vec4 clipPos = p_projections[0] * rotView * vec4(a_position, 1.0);

    gl_Position = clipPos.xyww;
    local_position = a_position;
}

#FRAGMENT

in vec3 local_position;
out vec4 color_out;

uniform samplerCube p_environment_map;
  
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
    vec3 envColor = texture(p_environment_map, local_position).rgb;
    
    envColor = envColor / (envColor + vec3(1.0));
    envColor = pow(envColor, vec3(1.0/2.2)); 
  
    vec3 dither = ScreenSpaceDither(gl_FragCoord.xy) * 4.0;

    color_out = vec4(envColor + dither, 1.0);
}