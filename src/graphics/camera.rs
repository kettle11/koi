use crate::*;

pub fn camera_plugin() -> Plugin {
    Plugin {
        pre_fixed_update_systems: vec![resize_camera.system()],
        ..Default::default()
    }
}

/// When a [Camera] is attached to an [Entity] with a [Transform] the camera's perspective
/// will be rendered to the default view.
/// Cameras can be configured to use different projections (perspective, orthographic, or custom).
/// Cameras also have a property `render_flags` that configures which [Entity]s are rendered.
#[derive(Clone, Debug, Component)]
pub struct Camera {
    pub enabled: bool,
    pub clear_color: Option<Color>,
    projection_matrix: Mat4,
    projection_mode: ProjectionMode,
    view_width: u32,
    view_height: u32,
    z_near: f32,
    z_far: f32,
    /// Only relevant for orthographic projections
    orthographic_height: f32,
    /// Only relevant for perspective projections.
    vertical_field_of_view_radians: f32,
    pub render_flags: RenderFlags,
    #[skip]
    pub camera_target: Option<CameraTarget>,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum CameraTarget {
    Primary,
    Window(kapp::WindowId),
    XRDevice(usize),
}

#[derive(Clone, Debug, SerializeDeserialize)]
pub enum ProjectionMode {
    Perspective,
    Orthographic,
    Custom(Mat4),
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new()
    }
}

impl Camera {
    pub fn new() -> Self {
        let mut camera = Self {
            projection_matrix: Mat4::IDENTITY,
            projection_mode: ProjectionMode::Perspective,
            view_width: 100,
            view_height: 100,
            enabled: true,
            clear_color: Some(Color::BLACK),
            z_near: 0.3,
            z_far: 300.0,
            orthographic_height: 1.0,
            vertical_field_of_view_radians: (72.0_f32).to_radians(),
            render_flags: RenderFlags::DEFAULT,
            camera_target: Some(CameraTarget::Primary),
        };
        camera.update_projection_matrix();
        camera
    }

    /// Creates a new [Camera] with an orthographic projection matrix.
    /// After it's created use [Camera::set_orthographic_height] to how large the camera's
    /// visible area is.
    pub fn with_orthographic_projection(mut self) -> Self {
        self.projection_mode = ProjectionMode::Orthographic;
        self.orthographic_height = 2.0;
        self.update_projection_matrix();
        self
    }

    /// Create's a new [Camera] with a custom projection matrix.
    ///  This may be useful when integrating with VR headsets, or when doing something custom.
    pub fn with_custom_projection(mut self, projection_matrix: Mat4) -> Self {
        self.projection_mode = ProjectionMode::Custom(projection_matrix);
        self
    }

    /// Creates a new camera configured to render the user-interface.
    pub fn new_for_user_interface() -> Self {
        let projection_matrix =
            kmath::projection_matrices::orthographic_gl(-1.0, 1.0, -1.0, 1.0, 0.0, 1.0);
        let mut camera = Self::new().with_custom_projection(projection_matrix);
        camera.render_flags = RenderFlags::USER_INTERFACE;
        camera.clear_color = None;
        camera.z_near = 0.0;
        camera
    }

    fn update_projection_matrix(&mut self) {
        let aspect_ratio = self.view_width as f32 / self.view_height as f32;
        self.projection_matrix = match self.projection_mode {
            ProjectionMode::Perspective => kmath::projection_matrices::perspective_infinite_gl(
                self.vertical_field_of_view_radians,
                aspect_ratio,
                self.z_near,
                //  self.z_far,
            ),

            ProjectionMode::Orthographic => {
                let width = aspect_ratio * self.orthographic_height;
                let half_width = width / 2.;
                let half_height = self.orthographic_height / 2.;

                kmath::projection_matrices::orthographic_gl(
                    -half_width,
                    half_width,
                    -half_height,
                    half_height,
                    self.z_near,
                    self.z_far,
                )
            }
            ProjectionMode::Custom(m) => m,
        };
    }

    pub fn set_view_size(&mut self, width: u32, height: u32) {
        if self.view_width != width || self.view_height != height {
            self.view_width = width;
            self.view_height = height;
            self.update_projection_matrix();
        }
    }

    pub fn get_view_size(&self) -> (u32, u32) {
        (self.view_width, self.view_height)
    }

    pub fn set_near_plane(&mut self, near_plane: f32) {
        self.z_near = near_plane;
        self.update_projection_matrix();
    }

    pub fn get_near_plane(&self) -> f32 {
        self.z_near
    }

    pub fn set_far_plane(&mut self, far_plane: f32) {
        self.z_far = far_plane;
        self.update_projection_matrix();
    }

    // Returns projection matrix
    pub fn projection_matrix(&self) -> Mat4 {
        self.projection_matrix
    }

    // Useful for shadow maps
    pub fn projection_matrix_with_z_near_and_z_far(&self, z_near: f32, z_far: f32) -> Mat4 {
        let aspect_ratio = self.view_width as f32 / self.view_height as f32;

        kmath::projection_matrices::perspective_gl(
            self.vertical_field_of_view_radians,
            aspect_ratio,
            z_near,
            z_far,
        )
    }

    /// Sets the vertical height (in world coordinates) which is visible to this camera.
    pub fn set_orthographic_height(&mut self, height: f32) {
        self.orthographic_height = height;
        self.update_projection_matrix();
    }

    pub fn set_vertical_field_of_view(&mut self, vertical_field_of_view_radians: f32) {
        self.vertical_field_of_view_radians = vertical_field_of_view_radians;
        self.update_projection_matrix();
    }

    pub fn set_projection_mode(&mut self, projection_mode: ProjectionMode) {
        self.projection_mode = projection_mode;
        self.update_projection_matrix();
    }

    /// Pass in view coordinates with 0,0 in the upper left and view_width, view_height in the bottom right.
    /// Creates a ray with its origin on the near clipping plane.
    pub fn view_to_ray(&self, transform: &crate::transform::Transform, x: f32, y: f32) -> Ray3 {
        let normalized = Vec2::new(x / self.view_width as f32, y / self.view_height as f32);
        // Convert to OpenGL coordinate space which is -1,-1 is bottom left, 1,1 is upper right
        let gl_space =
            (normalized * 2.0 + Vec2::new(-1.0, -1.0)).mul_by_component(Vec2::new(1.0, -1.0));

        let transform_matrix = transform.model() * self.projection_matrix.inversed();

        let gl_space_near = gl_space.extend(-1.0).extend(1.0);
        let gl_space_far = gl_space.extend(2.0).extend(1.0);

        let world_space_near = transform_matrix * gl_space_near;
        let world_space_far = transform_matrix * gl_space_far;

        let world_space_near = world_space_near.xyz() / world_space_near.w;
        let world_space_far = world_space_far.xyz() / world_space_far.w;

        Ray3::new(
            world_space_near,
            -(world_space_far - world_space_near).normalized(),
        )
    }
}
pub fn resize_camera(mut cameras: Query<(&mut Camera,)>, window: &NotSendSync<kapp::Window>) {
    // This is very incorrect, but it works for now with the single window assumption
    for camera in &mut cameras {
        let (width, height) = window.size();
        if width != 0 && height != 0 {
            camera.set_view_size(width, height);
        }
    }
}
