use crate::*;

pub fn camera_plugin() -> Plugin {
    Plugin {
        pre_fixed_update_systems: vec![resize_camera.system()],
        ..Default::default()
    }
}

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
    pub render_layers: RenderLayers,
    pub camera_target: Option<CameraTarget>,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum CameraTarget {
    Primary,
    Window(kapp::WindowId),
    XRDevice(usize),
}

#[derive(Clone, Debug)]
pub enum ProjectionMode {
    Perspective,
    Orthographic,
    Custom(Mat4),
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
            z_near: 1.0,
            z_far: 500.0,
            orthographic_height: 1.0,
            vertical_field_of_view_radians: (72.0_f32).to_radians(),
            render_layers: RenderLayers::DEFAULT,
            camera_target: Some(CameraTarget::Primary),
        };
        camera.update_projection_matrix();
        camera
    }

    pub fn new_orthographic() -> Self {
        let mut camera = Self {
            projection_matrix: Mat4::IDENTITY,
            projection_mode: ProjectionMode::Orthographic,
            view_width: 100,
            view_height: 100,
            enabled: true,
            clear_color: Some(Color::BLACK),
            z_near: 0.0,
            z_far: 500.0,
            orthographic_height: 2.0,
            vertical_field_of_view_radians: (60.0_f32).to_radians(),
            render_layers: RenderLayers::DEFAULT,
            camera_target: Some(CameraTarget::Primary),
        };
        camera.update_projection_matrix();
        camera
    }

    pub fn new_custom_projection_matrix(projection_matrix: Mat4) -> Self {
        Self {
            projection_matrix,
            projection_mode: ProjectionMode::Custom(projection_matrix),
            view_width: 100,
            view_height: 100,
            enabled: true,
            clear_color: Some(Color::BLACK),
            z_near: 0.0,
            z_far: 500.0,
            orthographic_height: 2.0,
            vertical_field_of_view_radians: (60.0_f32).to_radians(),
            render_layers: RenderLayers::DEFAULT,
            camera_target: Some(CameraTarget::Primary),
        }
    }

    /// Creates a new camera configured to render the user-interface.
    pub fn new_for_user_interface() -> Self {
        let projection_matrix =
            kmath::projection_matrices::orthographic_gl(-1.0, 1.0, -1.0, 1.0, -4.0, 4.0);
        let mut camera = Self::new_custom_projection_matrix(projection_matrix);
        camera.render_layers = RenderLayers::USER_INTERFACE;
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
    /// Does not yet work for an orthographic camera.
    /// The ray is in world space.
    pub fn view_to_ray(&self, transform: &crate::transform::Transform, x: f32, y: f32) -> Ray {
        let normalized = Vec2::new(x / self.view_width as f32, y / self.view_height as f32);
        // Convert to OpenGL coordinate space which is -1,-1 is bottom left, 1,1 is upper right
        let gl_space =
            (normalized * 2.0 + Vec2::new(-1.0, -1.0)).mul_by_component(Vec2::new(1.0, -1.0));
        let gl_space = gl_space.extend(-1.0);

        let camera_space = self.projection_matrix.inversed().transform_vector(gl_space);
        let camera_space = Vec3::new(camera_space[0], camera_space[1], -1.0);

        // Transform into world space.
        let camera_space = transform.rotation.rotate_vector3(camera_space);
        let direction = camera_space.normalized();

        Ray::new(transform.position, direction)
    }
}
pub fn resize_camera(mut cameras: Query<(&mut Camera,)>, window: &NotSendSync<kapp::Window>) {
    // This is very incorrect, but it works for now with the single window assumption
    for camera in &mut cameras {
        let (width, height) = window.size();
        camera.set_view_size(width, height);
    }
}
