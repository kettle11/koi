#VERTEX 

uniform mat4 p_views[NUM_VIEWS];
uniform mat4 p_projections[NUM_VIEWS];

in vec3 a_position;

out vec3 local_position;

void main()
{
    local_position = a_position;  
    gl_Position =  p_projections[0] * p_views[0] * vec4(a_position, 1.0);
}

#FRAGMENT

out vec4 color_out;
in vec3 local_position;

uniform sampler2D p_texture;

const vec2 invAtan = vec2(0.1591, 0.3183);

vec2 SampleSphericalMap(vec3 v)
{
    vec2 uv = vec2(atan(v.z, v.x), asin(v.y));
    uv *= invAtan;
    uv += 0.5;
    return uv;
}

void main()
{		
    vec2 uv = SampleSphericalMap(normalize(local_position)); // make sure to normalize localPos
    uv.y = 1.0 - uv.y;
    vec3 color = texture(p_texture, uv).rgb;
    color_out = vec4(color, 1.0);
}