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

// This is from the version on learnopengl.com

out vec4 color_out;
in vec3 local_position;

// The environment map being convoluted.
uniform samplerCube p_texture;

const float PI = 3.14159265359;

void main()
{		
	// The world vector acts as the normal of a tangent surface
    // from the origin, aligned to local_position. Given this normal, calculate all
    // incoming radiance of the environment. The result of this radiance
    // is the radiance of light coming from -Normal direction, which is what
    // we use in the PBR shader to sample irradiance.
    vec3 N = normalize(local_position);

    vec3 irradiance = vec3(0.0);   
    
    // tangent space calculation from origin point
    vec3 up    = vec3(0.0, 1.0, 0.0);
    vec3 right = normalize(cross(up, N));
    up         = normalize(cross(N, right));
       
    float sampleDelta = 0.025;
    float nrSamples = 0.0;
    for(float phi = 0.0; phi < 2.0 * PI; phi += sampleDelta)
    {
        for(float theta = 0.0; theta < 0.5 * PI; theta += sampleDelta)
        {
            // spherical to cartesian (in tangent space)
            vec3 tangentSample = vec3(sin(theta) * cos(phi),  sin(theta) * sin(phi), cos(theta));
            // tangent space to world
            vec3 sampleVec = tangentSample.x * right + tangentSample.y * up + tangentSample.z * N; 

            irradiance += texture(p_texture, sampleVec).rgb * cos(theta) * sin(theta);
            nrSamples++;
        }
    }
    irradiance = PI * irradiance * (1.0 / float(nrSamples));
    
    color_out = vec4(irradiance, 1.0);
}