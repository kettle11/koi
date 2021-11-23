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
  
void main()
{
    vec3 envColor = texture(p_environment_map, local_position).rgb;
    
    envColor = envColor / (envColor + vec3(1.0));
    envColor = pow(envColor, vec3(1.0/2.2)); 
  
    color_out = vec4(envColor, 1.0);
}