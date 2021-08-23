use std::collections::HashMap;
// use std::ops::{Deref, DerefMut};

use crate::graphics::texture::Texture;
use crate::*;
use kgraphics::*;

#[derive(Clone)]
pub struct Material {
    pub shader: Handle<Shader>,
    float_properties: HashMap<String, f32>,
    vec2_properties: HashMap<String, Vec2>,
    vec3_properties: HashMap<String, Vec3>,
    vec4_properties: HashMap<String, Vec4>,
    mat4_properties: HashMap<String, Mat4>,
    pub(crate) texture_properties: HashMap<String, (Handle<Texture>, u8)>,
    pub(crate) max_texture_unit: u8,
}

impl Material {
    pub const DEFAULT: Handle<Material> = Handle::new_with_just_index(0);
    pub const UNLIT: Handle<Material> = Handle::new_with_just_index(1);
    /// A fully emissive material
    pub const EMISSIVE: Handle<Material> = Handle::new_with_just_index(2);

    pub(crate) fn initialize_static_materials(materials: &mut Assets<Material>) {
        let mut unlit_material = Material::new(Shader::UNLIT);
        unlit_material.set_base_color(Color::WHITE);
        unlit_material.set_texture("p_base_color_texture", Texture::WHITE);
        unlit_material.set_vec2("p_texture_coordinate_offset", Vec2::ZERO);
        unlit_material.set_vec2("p_texture_coordinate_scale", Vec2::ONE);
        materials.add_and_leak(unlit_material, &Self::UNLIT);

        let mut emissive_material = Material::new(Handle::default());
        emissive_material.set_base_color(Color::WHITE);
        emissive_material.set_texture("p_base_color_texture", Texture::WHITE);
        emissive_material.set_vec3("p_emissive", Vec3::new(1.0, 1.0, 1.0));
        materials.add_and_leak(emissive_material, &Self::EMISSIVE);
    }

    pub fn new(shader: Handle<Shader>) -> Self {
        Self {
            shader,
            float_properties: HashMap::new(),
            vec2_properties: HashMap::new(),
            vec3_properties: HashMap::new(),
            vec4_properties: HashMap::new(),
            mat4_properties: HashMap::new(),
            texture_properties: HashMap::new(),
            max_texture_unit: 0,
        }
    }

    pub fn set_float(&mut self, name: &str, value: f32) {
        self.float_properties.insert(name.to_string(), value);
    }

    pub fn set_vec2(&mut self, name: &str, value: Vec2) {
        self.vec2_properties.insert(name.to_string(), value);
    }

    pub fn set_vec3(&mut self, name: &str, value: Vec3) {
        self.vec3_properties.insert(name.to_string(), value);
    }

    pub fn set_vec4(&mut self, name: &str, value: Vec4) {
        self.vec4_properties.insert(name.to_string(), value);
    }

    pub fn set_color(&mut self, name: &str, value: Color) {
        // For now just assume the shader's [ColorSpace] is linear sRGB.
        let RGBColor {
            red,
            green,
            blue,
            alpha,
        } = value.to_rgb_color(crate::color_spaces::LINEAR_SRGB);
        let value = Vec4::new(red, green, blue, alpha);
        self.vec4_properties.insert(name.to_string(), value);
    }

    pub fn set_mat4(&mut self, name: &str, value: Mat4) {
        self.mat4_properties.insert(name.to_string(), value);
    }

    pub fn set_texture(&mut self, name: &str, texture: Handle<Texture>) {
        let Self {
            texture_properties,
            max_texture_unit,
            ..
        } = self;
        let (texture_handle, _) = texture_properties
            .entry(name.to_string())
            .or_insert_with(|| {
                *max_texture_unit += 1;
                (Handle::default(), *max_texture_unit - 1)
            });
        *texture_handle = texture;
    }

    pub fn bind_material(
        &self,
        render_pass: &mut RenderPass,
        pipeline: &Pipeline,
        texture_assets: &Assets<Texture>,
    ) {
        for (name, value) in self.float_properties.iter() {
            if let Ok(property) = pipeline.get_float_property(name) {
                render_pass.set_float_property(&property, *value);
            } else {
                println!("WARNING: Shader does not have float property '{}'", name);
            }
        }
        for (name, value) in self.vec2_properties.iter() {
            if let Ok(property) = pipeline.get_vec2_property(name) {
                render_pass.set_vec2_property(&property, (*value).into());
            } else {
                println!("WARNING: Shader does not have Vec2 property '{}'", name);
            }
        }
        for (name, value) in self.vec3_properties.iter() {
            if let Ok(property) = pipeline.get_vec3_property(name) {
                render_pass.set_vec3_property(&property, (*value).into());
            } else {
                println!("WARNING: Shader does not have Vec3 property '{}'", name);
            }
        }
        for (name, value) in self.vec4_properties.iter() {
            if let Ok(property) = pipeline.get_vec4_property(name) {
                render_pass.set_vec4_property(&property, (*value).into());
            } else {
                println!("WARNING: Shader does not have Vec4 property '{}'", name);
            }
        }
        for (name, value) in self.mat4_properties.iter() {
            if let Ok(property) = pipeline.get_mat4_property(name) {
                render_pass.set_mat4_property(&property, (*value).as_array());
            } else {
                println!("WARNING: Shader does not have mat4 property '{}'", name);
            }
        }
        for (name, (texture, texture_unit)) in self.texture_properties.iter() {
            if let Ok(property) = pipeline.get_texture_property(name) {
                let texture = texture_assets.get(&texture);
                render_pass.set_texture_property(&property, Some(texture), *texture_unit);
            } else {
                println!("WARNING: Shader does not have texture property '{}'", name);
            }
        }
    }
}

pub struct MaterialAssetLoader {}
impl AssetLoader<Material> for MaterialAssetLoader {
    fn new() -> Self {
        Self {}
    }

    fn load_with_options(
        &mut self,
        _path: &str,
        _handle: Handle<Material>,
        _options: <Material as LoadableAssetTrait>::Options,
    ) {
        unimplemented!()
    }
}
impl LoadableAssetTrait for Material {
    type Options = ();
    type AssetLoader = MaterialAssetLoader;
}

/// Some built in properties for materials
impl Material {
    pub fn set_base_color(&mut self, color: Color) {
        self.set_color("p_base_color", color)
    }
}
