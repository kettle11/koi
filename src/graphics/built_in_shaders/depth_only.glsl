#VERTEX 

in vec3 a_position;

uniform mat4 p_model;
uniform mat4 p_views[1];
uniform mat4 p_projections[1];

void main()
{
    gl_Position = p_projections[0] * p_views[0] * p_model * vec4(a_position, 1.0);
    // Clamp things outside near clipping plane to be on near clipping plane.
    // gl_Position.z = max(gl_Position.z, 0.0);  
}

#FRAGMENT

void main()
{
}