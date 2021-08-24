use crate::*;

#[derive(Debug, Clone)]
pub struct PBRProperties {
    pub base_color: Color,
    pub base_color_texture: Option<Handle<Texture>>,
    pub metallic: f32,
    pub roughness: f32,
    pub metallic_roughness_texture: Option<Handle<Texture>>,
    pub ambient: f32,
    pub ambient_texture: Option<Handle<Texture>>,
    pub emissive: Color,
    pub emissive_texture: Option<Handle<Texture>>,
    pub normal_texture: Option<Handle<Texture>>,
    pub blending: Option<(BlendFactor, BlendFactor)>,
}

impl Default for PBRProperties {
    fn default() -> Self {
        Self {
            base_color: Color::WHITE,
            base_color_texture: Some(Texture::WHITE),
            metallic: 0.0,
            roughness: 0.8,
            metallic_roughness_texture: Some(Texture::WHITE),
            ambient: 1.0,
            ambient_texture: Some(Texture::WHITE),
            normal_texture: Some(Texture::NORMAL),
            emissive_texture: Some(Texture::WHITE),
            emissive: Color::BLACK,
            blending: None,
        }
    }
}

pub fn new_pbr_material(shader: Handle<Shader>, pbr_properties: PBRProperties) -> Material {
    let mut material = Material::new(shader);
    material.set_color("p_base_color", pbr_properties.base_color);
    let base_color_texture = pbr_properties
        .base_color_texture
        .clone()
        .unwrap_or(Texture::WHITE);
    material.set_texture("p_base_color_texture", base_color_texture);

    material.set_float("p_metallic", pbr_properties.metallic);
    material.set_float("p_roughness", pbr_properties.roughness);

    let metallic_roughness_texture = pbr_properties
        .metallic_roughness_texture
        .clone()
        .unwrap_or(Texture::WHITE);
    material.set_texture("p_metallic_roughness_texture", metallic_roughness_texture);

    material.set_float("p_ambient", pbr_properties.ambient);
    let ambient_texture = pbr_properties
        .ambient_texture
        .clone()
        .unwrap_or(Texture::WHITE);
    material.set_texture("p_ambient_texture", ambient_texture);

    let normal_texture = pbr_properties
        .normal_texture
        .clone()
        .unwrap_or(Texture::NORMAL);
    material.set_texture("p_normal_texture", normal_texture);

    material.set_vec3("p_emissive", {
        let rgb_color = pbr_properties
            .emissive
            .to_rgb_color(color_spaces::LINEAR_SRGB);
        Vec3::new(rgb_color.red, rgb_color.green, rgb_color.blue)
    });
    let p_emissive_texture = pbr_properties.emissive_texture.unwrap_or(Texture::WHITE);
    material.set_texture("p_emissive_texture", p_emissive_texture);

    material
}
