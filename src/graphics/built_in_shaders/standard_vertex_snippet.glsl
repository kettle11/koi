#ifdef MULTVIEW
#extension GL_OVR_multiview2 : require
layout (num_views = 2) in;
#endif

uniform mat4 p_views[NUM_VIEWS];
uniform mat4 p_projections[NUM_VIEWS];

in vec3 a_position;
in vec2 a_texture_coordinate;
in vec3 a_normal;

uniform mat4 p_model;

out vec2 TexCoords;
out vec3 WorldPosition;
out vec3 Normal;

void main()
{
    WorldPosition = vec3(p_model * vec4(a_position, 1.0));
    Normal = mat3(p_model) * a_normal;
    TexCoords = a_texture_coordinate;

    #ifdef MULTVIEW
        mat4 view = p_views[gl_ViewID_OVR];
        mat4 projection = p_projections[gl_ViewID_OVR];
    #else 
        mat4 view = p_views[0];
        mat4 projection = p_projections[0];
    #endif
    
    // For now share the same projection matrix between views.
    gl_Position = projection * view * p_model * vec4(a_position, 1.0);
}