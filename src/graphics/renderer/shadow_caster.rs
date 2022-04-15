use super::*;

// A shadow caster for a light.
#[derive(NotCloneComponent)]
pub struct ShadowCaster {
    pub shadow_cascades: Vec<ShadowCascadeInfo>,
    pub(crate) texture_size: u32,
    pub ibl_shadowing: f32,
}

impl ShadowCaster {
    pub fn new() -> Self {
        Self {
            shadow_cascades: Vec::new(),
            texture_size: 2048,
            ibl_shadowing: 0.0,
        }
    }

    pub fn with_ibl_shadowing(mut self, ibl_shadowing: f32) -> Self {
        self.ibl_shadowing = ibl_shadowing;
        self
    }
}

pub struct ShadowCascadeInfo {
    pub offscreen_render_target: OffscreenRenderTarget,
    pub(crate) world_to_light_space: Mat4,
}

impl ShadowCaster {
    pub fn prepare_shadow_casting(
        &mut self,
        graphics: &mut Graphics,
        textures: &mut Assets<Texture>,
    ) {
        if self.shadow_cascades.is_empty() {
            // Setup shadow textures
            for _ in 0..4 {
                let offscreen_render_target = OffscreenRenderTarget::new(
                    graphics,
                    textures,
                    Vec2u::new(self.texture_size as _, self.texture_size as _),
                    None,
                    Some((
                        kgraphics::PixelFormat::Depth32F,
                        TextureSettings {
                            // Nearest because this will not have mipmaps generated
                            minification_filter: kgraphics::FilterMode::Nearest,
                            magnification_filter: kgraphics::FilterMode::Nearest,
                            wrapping_horizontal: kgraphics::WrappingMode::ClampToEdge, // This should be clamp to border but that's not supported on WebGL
                            wrapping_vertical: kgraphics::WrappingMode::ClampToEdge, // This should be clamp to border but that's not supported on WebGL
                            srgb: false,
                            generate_mipmaps: false,
                            ..Default::default()
                        },
                    )),
                );

                self.shadow_cascades.push(ShadowCascadeInfo {
                    offscreen_render_target,
                    world_to_light_space: Mat4::ZERO, // This gets set later.
                });
            }
        }
    }
}

pub fn render_shadow_pass(
    shaders: &Assets<Shader>,
    meshes: &Assets<Mesh>,
    command_buffer: &mut CommandBuffer,
    camera: &Camera,
    camera_global_transform: &GlobalTransform,
    lights: &mut Lights,
    renderables: &Renderables,
    cascade_depths: &[f32; 4],
) {
    let camera_view_inversed = camera_global_transform.model();

    let mut z_near = camera.get_near_plane();
    let mut camera_clip_space_to_world = [Mat4::ZERO; 4];
    for (i, z_far) in cascade_depths.iter().enumerate() {
        // The +1.0 to z_far here prevents an issue where lines appear between cascades.
        let projection = camera.projection_matrix_with_z_near_and_z_far(z_near, z_far + 1.0);
        camera_clip_space_to_world[i] = camera_view_inversed * projection.inversed();
        z_near = *z_far;
    }

    // In the future this could be reduced to light's that area of influence overlaps the camera's frustum.
    for (light_global_transform, _light, shadow_caster) in lights {
        if let Some(shadow_caster) = shadow_caster {
            // Render shadow map cascades
            for (i, cascade) in shadow_caster.shadow_cascades.iter_mut().enumerate() {
                let view_matrix = light_global_transform.model().inversed();
                let camera_to_light_space = view_matrix * camera_clip_space_to_world[i];

                // Is negative z correct here?
                let corners = [
                    camera_to_light_space * (Vec4::new(-1., -1., 1., 1.)),
                    camera_to_light_space * (Vec4::new(1., -1., 1., 1.)),
                    camera_to_light_space * (Vec4::new(1., 1., 1., 1.)),
                    camera_to_light_space * (Vec4::new(-1., 1., 1., 1.)),
                    camera_to_light_space * (Vec4::new(-1., -1., -1., 1.)),
                    camera_to_light_space * (Vec4::new(1., -1., -1., 1.)),
                    camera_to_light_space * (Vec4::new(1., 1., -1., 1.)),
                    camera_to_light_space * (Vec4::new(-1., 1., -1., 1.)),
                ];

                let corners = [
                    corners[0].xyz() / corners[0].w,
                    corners[1].xyz() / corners[1].w,
                    corners[2].xyz() / corners[2].w,
                    corners[3].xyz() / corners[3].w,
                    corners[4].xyz() / corners[4].w,
                    corners[5].xyz() / corners[5].w,
                    corners[6].xyz() / corners[6].w,
                    corners[7].xyz() / corners[7].w,
                ];

                // Clamp the shadow map bounding box to texel edges to reduce shimmering
                let bounding_box = Box3::from_points(corners);
                let world_units_per_texel = bounding_box.size() / shadow_caster.texture_size as f32;
                let min = (bounding_box.min.div_by_component(world_units_per_texel))
                    .floor()
                    .mul_by_component(world_units_per_texel);
                let max = (bounding_box.max.div_by_component(world_units_per_texel))
                    .floor()
                    .mul_by_component(world_units_per_texel);
                let bounding_box = Box3 { min, max };

                // The light's matrix must enclose these.

                // It would be better to detect objects within the light's bounds and determine where the near
                // and far planes should go appropriately.
                let shadow_behind_light = 300.;
                let projection_matrix = kmath::projection_matrices::orthographic_gl(
                    bounding_box.min[0],
                    bounding_box.max[0],
                    bounding_box.min[1],
                    bounding_box.max[1],
                    -bounding_box.max[2] - shadow_behind_light, // I don't understand why these need to be negated and reversed
                    -bounding_box.min[2] + shadow_behind_light,
                );

                // println!("PROJECTION MATRIX{:#?}", projection_matrix);

                cascade.world_to_light_space = projection_matrix * view_matrix;

                render_depth_only(
                    shaders,
                    meshes,
                    command_buffer,
                    cascade.offscreen_render_target.framebuffer(),
                    &view_matrix,
                    &projection_matrix,
                    shadow_caster.texture_size,
                    renderables,
                );
            }
        }
    }
}
