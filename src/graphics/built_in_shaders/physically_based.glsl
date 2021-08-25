#VERTEX 

in vec3 a_position;
in vec2 a_texture_coordinate;
in vec3 a_normal;

uniform mat4 p_model;
uniform mat4 p_view;
uniform mat4 p_projection;

out vec2 TexCoords;
out vec3 WorldPosition;
out vec3 Normal;

void main()
{
    WorldPosition = vec3(p_model * vec4(a_position, 1.0));
    Normal = mat3(p_model) * a_normal;
    TexCoords = a_texture_coordinate;
    gl_Position = p_projection * p_view * p_model * vec4(a_position, 1.0);
}

#FRAGMENT

in vec2 TexCoords;
in vec3 WorldPosition;  
in vec3 Normal;

out vec4 color_out;

// Physically based rendering properties.
// These adjust the entire model.
uniform vec4 p_base_color;
uniform float p_metallic;
uniform float p_roughness;
// How much ambient light is visible to this model.
uniform float p_ambient;
// Does this item produce its own light?
uniform vec3 p_emissive; 

// Physically based rendering textures
// These are multipled by the corresponding properties.
uniform sampler2D p_base_color_texture;
uniform sampler2D p_metallic_roughness_texture;

uniform sampler2D p_normal_texture;
uniform sampler2D p_ambient_texture;
uniform sampler2D p_emissive_texture;

uniform vec3 p_camera_position;

uniform int p_light_count;

uniform float p_dither_scale;

uniform vec3 p_fog_color;
uniform float p_fog_start;
uniform float p_fog_end;

struct Light {
    vec3 position;
    vec3 direction;
    int mode;
    vec3 color_and_intensity;
    float radius;
    int shadows_enabled;
    float ambient;
    // Ambient light added to shadows only
    // vec3 shadow_color;
};  

uniform Light p_lights[4];

// sampler2Ds can't be in structs (some drivers support it, but not all)
// so store them separately.
// Eventually it'd be better to combine light shadow maps together.
uniform sampler2D p_light_shadow_maps_0;
uniform sampler2D p_light_shadow_maps_1;
uniform sampler2D p_light_shadow_maps_2;
uniform sampler2D p_light_shadow_maps_3;

uniform mat4 p_world_to_light_space_0;
uniform mat4 p_world_to_light_space_1;
uniform mat4 p_world_to_light_space_2;
uniform mat4 p_world_to_light_space_3;

// Up to 4 cascades are supported.
// uniform float p_shadow_cascades[4];

// ------- Copied from learnopengl (for now) ------------

const float PI = 3.14159265359;

// ----------------------------------------------------------------------------
// Easy trick to get tangent-normals to world-space to keep PBR code simplified.
// Don't worry if you don't get what's going on; you generally want to do normal 
// mapping the usual way for performance anways; I do plan make a note of this 
// technique somewhere later in the normal mapping tutorial.
vec3 getNormalFromMap()
{
    vec3 tangentNormal = texture(p_normal_texture, TexCoords).xyz * 2.0 - 1.0;

    vec3 Q1  = dFdx(WorldPosition);
    vec3 Q2  = dFdy(WorldPosition);
    vec2 st1 = dFdx(TexCoords);
    vec2 st2 = dFdy(TexCoords);

    vec3 N   = normalize(Normal);
    
    // This line means that meshes without UVs will not be able to calculate their normal.
    vec3 T  = normalize(Q1*st2.t - Q2*st1.t);
    vec3 B  = -normalize(cross(N, T));
    mat3 TBN = mat3(T, B, N);

    return normalize(TBN * tangentNormal);
  // return N;
}

// ----------------------------------------------------------------------------
float DistributionGGX(vec3 N, vec3 H, float roughness)
{
    float a = roughness*roughness;
    float a2 = a*a;
    float NdotH = max(dot(N, H), 0.0);
    float NdotH2 = NdotH*NdotH;

    float nom   = a2;
    float denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    // Is code needed here to prevent a divide by zero?
    return nom / denom;
}
// ----------------------------------------------------------------------------
float GeometrySchlickGGX(float NdotV, float roughness)
{
    float r = (roughness + 1.0);
    float k = (r*r) / 8.0;

    float nom   = NdotV;
    float denom = NdotV * (1.0 - k) + k;

    return nom / denom;
}
// ----------------------------------------------------------------------------
float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness)
{
    float NdotV = max(dot(N, V), 0.0);
    float NdotL = max(dot(N, L), 0.0);
    float ggx2 = GeometrySchlickGGX(NdotV, roughness);
    float ggx1 = GeometrySchlickGGX(NdotL, roughness);

    return ggx1 * ggx2;
}
// ----------------------------------------------------------------------------
vec3 fresnelSchlick(float cosTheta, vec3 F0)
{
    // A comment on LearnOpenGL indicated that cosTheta can sometimes be larger than 1.0 and 
    // cause pow to return NaN. 
    // It'd be worth testing for later.
   // cosTheta = min(cosTheta, 1.0);
    return F0 + (1.0 - F0) * pow(1.0 - cosTheta, 5.0);
}
// ----------------------------------------------------------------------------

// Portal 2 Screenspace dithering (modified for VR):
// http://media.steampowered.com/apps/valve/2015/Alex_Vlachos_Advanced_VR_Rendering_GDC2015.pdf
vec3 ScreenSpaceDither( vec2 vScreenPos )
{
    vec3 vDither = vec3(dot( vec2( 171.0, 231.0 ), vScreenPos + 0.0 )); // the 0.0 should be time
    vDither.rgb = fract( vDither.rgb / vec3( 103.0, 71.0, 97.0 ) ) - vec3( 0.5, 0.5, 0.5 );
    return ( vDither.rgb / 255.0 ) * 0.375;
}

float ShadowCalculation(in sampler2D shadowMap, vec4 fragPosLightSpace, vec3 lightDir)
{
    // perform perspective divide
    vec3 projCoords = fragPosLightSpace.xyz / fragPosLightSpace.w;
    // transform to [0,1] range
    projCoords = projCoords * 0.5 + 0.5;
    // get closest depth value from light's perspective (using [0,1] range fragPosLight as coords)
   // float closestDepth = texture(shadowMap, projCoords.xy).r; 
    // get depth of d fragment from light's perspective
    float currentDepth = projCoords.z;

   // These lines are screwed up, but should be fixed to reduce peter-panning.
   // vec3 normal = normalize(Normal);
   // float bias = max(0.05 * (1.0 - dot(normal, lightDir)), 0.005);

    // check whether d frag pos is in shadow
   // float shadow = currentDepth - 0.002 > closestDepth  ? 1.0 : 0.0;

    float bias = 0.002;
    float shadow = 0.0;
    vec2 texelSize = 1.0 / vec2(textureSize(shadowMap, 0));

    // Percentage-close filtering (PCF)
    // This could be improved in the future by taking fewer dithered samples.
    for(int x = -1; x <= 1; ++x)
    {
        for(int y = -1; y <= 1; ++y)
        {
            float pcfDepth = texture(shadowMap, projCoords.xy + vec2(x, y) * texelSize).r; 
            shadow += currentDepth - bias > pcfDepth ? 1.0 : 0.0;        
        }    
    }
    shadow /= 9.0;

    if(projCoords.z > 1.0)
        shadow = 0.0;

    return shadow;
}


void main()
{
    float z = gl_FragCoord.z / gl_FragCoord.w;
    float fog_factor = (z - p_fog_start) / (p_fog_end - p_fog_start);
    fog_factor = clamp(fog_factor, 0.0, 1.0 );
    fog_factor = 0.0;
    
    vec3 color = p_fog_color;
    float alpha = 1.0;

    // reflectance equation
    vec3 Lo = vec3(0.0);
    
    vec3 emissive = p_emissive * texture(p_emissive_texture, TexCoords).rgb;
    vec3 debug_color = vec3(0.0);

    if (fog_factor != 1.0) {
        vec4 metallic_roughness = texture(p_metallic_roughness_texture, TexCoords);
        vec4 base_color_rgba = (p_base_color * texture(p_base_color_texture, TexCoords));
        vec3 base_color = base_color_rgba.rgb;
        alpha = base_color_rgba.a;

        float metallic  = p_metallic * metallic_roughness.b;
        float roughness = p_roughness * metallic_roughness.g;
        float ambient_amount = p_ambient * texture(p_ambient_texture, TexCoords).r;

    // vec3 base_color = (p_base_color).rgb;
    //  float metallic  = 1.0 - p_metallic;
    //  float roughness = p_roughness;


        vec3 N = getNormalFromMap();
        vec3 V = normalize(p_camera_position - WorldPosition);

        // calculate reflectance at normal incidence; if dia-electric (like plastic) use F0 
        // of 0.04 and if it's a metal, use the albedo color as F0 (metallic workflow)    
        vec3 F0 = vec3(0.04); 
        F0 = mix(F0, base_color, metallic);

    // Reflection direction
        vec3 R = reflect(-V, N);

        vec3 ambient_light = vec3(0.0);


        for(int i = 0; i < p_light_count; ++i) 
        {
            Light light = p_lights[i];        

            // calculate per-light radiance
            vec3 L;// = light.position - WorldPosition;
        // vec3 center_to_ray = (dot(L, R) * R) - L;
        // vec3 closest_point = light.position + center_to_ray * clamp( light.radius / length( center_to_ray ), 0.0, 1.0 );


            if (light.mode == 0) {
                L = normalize(-light.direction);
            } else if (light.mode == 1) {
                L = normalize(light.position - WorldPosition);
            }

            // Flipping the comment out lines changes area lights on and off.
        // L = normalize(closest_point - WorldPosition);
        //  L = normalize(light.position - WorldPosition);
        //  L = normalize(-light.direction);
        //  float distance = length(closest_point - WorldPosition);
            float distance = length(light.position - WorldPosition);

            vec3 H = normalize(V + L);
            float attenuation;
            
            if (light.mode == 0) {
                attenuation = 1.0;
            } else if (light.mode == 1) {
                attenuation = 1.0 / (distance * distance);
            }

        //   debugColor = vec3(distance) / 30.;


            vec3 radiance = light.color_and_intensity * attenuation;
            // Cook-Torrance BRDF
            float NDF = DistributionGGX(N, H, roughness);   
            float G   = GeometrySmith(N, V, L, roughness);      
            vec3 F    = fresnelSchlick(clamp(dot(H, V), 0.0, 1.0), F0);
            
            vec3 nominator    = NDF * G * F; 

            float denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0);
            vec3 specular = nominator / max(denominator, 0.001); // prevent divide by zero for NdotV=0.0 or NdotL=0.0
            
            // kS is equal to Fresnel
            vec3 kS = F;
            // for energy conservation, the diffuse and specular light can't
            // be above 1.0 (unless the surface emits light); to preserve this
            // relationship the diffuse component (kD) should equal 1.0 - kS.
            vec3 kD = vec3(1.0) - kS;
            // multiply kD by the inverse metalness such that only non-metals 
            // have diffuse lighting, or a linear blend if partly metal (pure metals
            // have no diffuse light).
            kD *= 1.0 - metallic;	  

            // scale light by NdotL
            float NdotL = max(dot(N, L), 0.0);        

            float shadow = 0.0;

            if (p_lights[i].shadows_enabled == 1) {
                if (z > 140.) {
                    vec4 light_space_position = p_world_to_light_space_3 * vec4(WorldPosition, 1.0);
                    shadow = ShadowCalculation(p_light_shadow_maps_3, light_space_position, L);
                    //debug_color = vec3(1.0, 0.0, 0.0);
                } else if (z > 60.) {
                    vec4 light_space_position = p_world_to_light_space_2 * vec4(WorldPosition, 1.0);
                    shadow = ShadowCalculation(p_light_shadow_maps_2, light_space_position, L);
                    //debug_color = vec3(0.0, 1.0, 0.0);
                } else if (z > 20.) {
                    vec4 light_space_position = p_world_to_light_space_1 * vec4(WorldPosition, 1.0);
                    shadow = ShadowCalculation(p_light_shadow_maps_1, light_space_position, L);
                    //debug_color = vec3(0.0, 0.0, 1.0);
                } else {
                    vec4 light_space_position = p_world_to_light_space_0 * vec4(WorldPosition, 1.0);
                    shadow = ShadowCalculation(p_light_shadow_maps_0, light_space_position, L);
                }
            }

            ambient_light += radiance * light.ambient;
            
            // add to outgoing radiance Lo
            Lo += (kD * base_color / PI + specular) * radiance * NdotL * (1.0 - shadow);  // note that we already multiplied the BRDF by the Fresnel (kS) so we won't multiply by kS again
            
            // Add ambient light only to the shadow
            // Lo += shadow * light.shadow_color;
        }   

        // ambient lighting (note that the next IBL tutorial will replace 
        // this ambient lighting with environment lighting).
        vec3 ambient = ambient_light * base_color * ambient_amount;
        color = ambient + Lo;//+ emissive;
    }
    
    // This should be applied before the shader instead.
  //  color = mix(color, p_fog_color, fog_factor );
    color += emissive;

  //  color += debug_color * 0.6;

    // HDR tonemapping
   // color = color / (color + vec3(1.0));

    // gamma correct
    vec3 dither = ScreenSpaceDither(gl_FragCoord.xy) * p_dither_scale;
    
   // color = pow(color, vec3(1.0/2.2)) + dither; 
        
    color_out = vec4(color, alpha);
}