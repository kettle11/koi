use kmath::*;

use crate::texture_atlas::*;
use crate::*;

pub struct Drawer {
    pub first_mesh: DrawerMeshData,
    pub second_mesh: DrawerMeshData,
    pub(crate) view_width: f32,
    pub(crate) view_height: f32,
    pub texture_atlas: TextureAtlas,
    pub images: Images,
    pub(crate) clipping_mask: Vec<Box2>,
}

pub struct DrawerMeshData {
    pub positions: Vec<Vec3>,
    pub texture_coordinates: Vec<Vec2>,
    pub colors: Vec<Vec4>,
    pub indices: Vec<[u32; 3]>,
}

impl Default for DrawerMeshData {
    fn default() -> Self {
        Self::new()
    }
}

impl DrawerMeshData {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            texture_coordinates: Vec::new(),
            colors: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.positions.clear();
        self.texture_coordinates.clear();
        self.colors.clear();
        self.indices.clear();
    }
    pub fn extend_indices(&mut self, indices: &[[u32; 3]]) {
        self.indices
            .extend(indices.iter().map(|v| [v[2], v[1], v[0]]))
    }
}
impl Default for Drawer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawer {
    pub fn new() -> Self {
        Self {
            first_mesh: DrawerMeshData::new(),
            second_mesh: DrawerMeshData::new(),
            view_width: 100.,
            view_height: 100.,
            texture_atlas: TextureAtlas::new(1024),
            images: Images::new(),
            clipping_mask: vec![Box2::new(-Vec2::MAX, Vec2::MAX)],
        }
    }

    pub fn set_view_width_height(&mut self, width: f32, height: f32) {
        self.view_width = width;
        self.view_height = height;
    }

    pub fn push_clipping_mask(&mut self, clipping_mask: Box2) {
        self.clipping_mask.push(clipping_mask);
    }

    pub fn pop_clipping_mask(&mut self) {
        self.clipping_mask.pop();
    }

    pub fn reset(&mut self) {
        self.first_mesh.reset();
        self.second_mesh.reset();
        self.clipping_mask.clear();
        self.clipping_mask.push(Box2::new(-Vec2::MAX, Vec2::MAX));
    }

    pub fn glyph_position(offset: Vec3, scale: f32, c: &fontdue::layout::GlyphPosition) -> Box2 {
        let x = c.x / scale + offset.x;
        let y = c.y / scale + offset.y;

        let width = c.width as f32 / scale;
        let height = c.height as f32 / scale;
        Box2 {
            min: Vec2::new(x, y),
            max: Vec2::new(x + width, y + height),
        }
    }

    pub fn text(
        &mut self,
        fontdue_font: &fontdue::Font,
        layout: &mut fontdue::layout::Layout,
        offset: Vec3,
        color: Color,
        scale: f32,
    ) {
        let z = offset.z;
        for c in layout.glyphs() {
            let atlas_rectangle = self
                .texture_atlas
                .get_character(fontdue_font, c.key)
                .unwrap();

            let atlas_rectangle = Box2::new_with_min_corner_and_size(
                Vec2::new(
                    atlas_rectangle.x as f32 / self.texture_atlas.width as f32,
                    atlas_rectangle.y as f32 / self.texture_atlas.height as f32,
                ),
                Vec2::new(
                    atlas_rectangle.width as f32 / self.texture_atlas.width as f32,
                    atlas_rectangle.height as f32 / self.texture_atlas.height as f32,
                ),
            );

            let glyph_position = Self::glyph_position(offset, scale, c);

            let offset = self.first_mesh.positions.len() as u32;

            let corners = glyph_position.corners();
            self.first_mesh.positions.extend_from_slice(&[
                self.position_to_gl(corners[0].extend(z)),
                self.position_to_gl(corners[1].extend(z)),
                self.position_to_gl(corners[2].extend(z)),
                self.position_to_gl(corners[3].extend(z)),
            ]);

            self.first_mesh.texture_coordinates.extend_from_slice(&[
                Vec2::new(atlas_rectangle.min.x, atlas_rectangle.min.y),
                Vec2::new(atlas_rectangle.max.x, atlas_rectangle.min.y),
                Vec2::new(atlas_rectangle.max.x, atlas_rectangle.max.y),
                Vec2::new(atlas_rectangle.min.x, atlas_rectangle.max.y),
            ]);

            let color = color.to_srgb();
            self.first_mesh
                .colors
                .extend_from_slice(&[color, color, color, color]);
            self.first_mesh.extend_indices(&[
                [offset, offset + 1, offset + 2],
                [offset, offset + 2, offset + 3],
            ]);
        }
    }

    pub fn image(&mut self, rectangle: Box3, image_handle: &ImageHandle) {
        let _z = rectangle.max.z;
        let rectangle = Box2 {
            min: rectangle.min.xy(),
            max: rectangle.max.xy(),
        };
        let clipped_rectangle = self.clip_rectangle(rectangle);

        if clipped_rectangle.area() > 0.0 {
            if let Some(atlas_rectangle) = self.images.get_image_texture_rect(image_handle) {
                let atlas_rectangle = Box2::new_with_min_corner_and_size(
                    Vec2::new(
                        atlas_rectangle.x as f32 / self.images.get_size() as f32,
                        atlas_rectangle.y as f32 / self.images.get_size() as f32,
                    ),
                    Vec2::new(
                        atlas_rectangle.width as f32 / self.images.get_size() as f32,
                        atlas_rectangle.height as f32 / self.images.get_size() as f32,
                    ),
                );

                let offset = self.second_mesh.positions.len() as u32;

                let (width, height) = clipped_rectangle.size().xy().into();
                let (x, y) = clipped_rectangle.min.into();

                self.second_mesh.positions.extend_from_slice(&[
                    self.position_to_gl(Vec3::new(x, y, 0.0)),
                    self.position_to_gl(Vec3::new(x + width, y, 0.0)),
                    self.position_to_gl(Vec3::new(x + width, y + height, 0.0)),
                    self.position_to_gl(Vec3::new(x, y + height, 0.0)),
                ]);

                // Adjust texture coordinates based on how the rectangle was clipped.
                let texture_coordinate_min =
                    (clipped_rectangle.min - rectangle.min).div_by_component(rectangle.size());
                let size = clipped_rectangle.size().div_by_component(rectangle.size());

                let atlas_rectangle_size = atlas_rectangle.size();
                let min = atlas_rectangle.min
                    + atlas_rectangle_size.mul_by_component(texture_coordinate_min);
                let max = min + size.mul_by_component(atlas_rectangle_size);
                let atlas_rectangle = Box2::new(min, max);

                self.second_mesh.texture_coordinates.extend_from_slice(&[
                    Vec2::new(atlas_rectangle.min.x, atlas_rectangle.min.y),
                    Vec2::new(atlas_rectangle.max.x, atlas_rectangle.min.y),
                    Vec2::new(atlas_rectangle.max.x, atlas_rectangle.max.y),
                    Vec2::new(atlas_rectangle.min.x, atlas_rectangle.max.y),
                ]);

                let color = Vec4::ONE;
                self.second_mesh
                    .colors
                    .extend_from_slice(&[color, color, color, color]);
                self.second_mesh.extend_indices(&[
                    [offset, offset + 1, offset + 2],
                    [offset, offset + 2, offset + 3],
                ]);
            }
        }
    }

    pub fn current_clipping_mask(&self) -> Box2 {
        *self.clipping_mask.last().unwrap()
    }

    fn clip_rectangle(&mut self, rectangle: Box2) -> Box2 {
        rectangle.intersection(self.current_clipping_mask())
    }

    /// Returns the rectangle that will actually be displayed.
    pub fn rectangle(&mut self, rectangle: Box3, color: Color) -> Box2 {
        let _z = rectangle.max.z;
        let rectangle = Box2 {
            min: rectangle.min.xy(),
            max: rectangle.max.xy(),
        };
        let rectangle = self.clip_rectangle(rectangle);

        if color.alpha == 0.0 {
            return rectangle;
        }
        if rectangle.area() != 0.0 {
            let color = color.to_srgb();
            let (width, height) = rectangle.size().xy().into();
            let (x, y) = rectangle.min.into();

            let offset = self.first_mesh.positions.len() as u32;
            self.first_mesh.positions.extend_from_slice(&[
                self.position_to_gl(Vec3::new(x, y, 0.0)),
                self.position_to_gl(Vec3::new(x + width, y, 0.0)),
                self.position_to_gl(Vec3::new(x + width, y + height, 0.0)),
                self.position_to_gl(Vec3::new(x, y + height, 0.0)),
            ]);
            self.first_mesh.texture_coordinates.extend_from_slice(&[
                Vec2::ZERO,
                Vec2::ZERO,
                Vec2::ZERO,
                Vec2::ZERO,
            ]);

            //   let current_color = Vec4::new(1.0, 1.0, 1.0, 1.0);
            self.first_mesh
                .colors
                .extend_from_slice(&[color, color, color, color]);
            self.first_mesh.extend_indices(&[
                [offset, offset + 1, offset + 2],
                [offset, offset + 2, offset + 3],
            ]);
        }
        rectangle
    }

    fn position_to_gl(&self, mut position: Vec3) -> Vec3 {
        let clipping_mask = self.current_clipping_mask();
        position.x = clipping_mask.min.x.max(position.x).min(clipping_mask.max.x);
        position.y = clipping_mask.min.y.max(position.y).min(clipping_mask.max.y);

        let x = (position.x / self.view_width) * 2.0 - 1.0;
        let y = (position.y / self.view_height) * -2.0 + 1.0;
        Vec3::new(x, y, 0.0)
    }

    fn push_position(&mut self, position: Vec3) {
        self.first_mesh
            .positions
            .push(self.position_to_gl(position));
    }

    fn corner(
        &mut self,
        radius: f32,
        center_index: u32,
        corner_center: Vec3,
        start_angle: f32,
        color: Vec4,
    ) {
        let mut angle = start_angle;
        let steps = 20;
        let step_amount = (std::f32::consts::PI / 2.0) / (steps - 1) as f32;
        for i in 0..steps {
            let len = self.first_mesh.positions.len() as u32;
            if i != 0 {
                self.first_mesh
                    .extend_indices(&[[center_index, len - 1, len]]);
            }
            let (sin, cos) = angle.sin_cos();
            let position = corner_center + Vec3::new(cos, sin, 0.0) * radius;

            self.push_position(position);
            self.first_mesh.colors.push(color);

            self.first_mesh.texture_coordinates.push(Vec2::ZERO);
            angle += step_amount;
        }
    }

    /// Returns the rectangle that will actually be displayed.
    pub fn rounded_rectangle(
        &mut self,
        rectangle: Box3,
        corner_radius: Vec4,
        color: Color,
    ) -> Box2 {
        if corner_radius == Vec4::fill(0.0) {
            return self.rectangle(rectangle, color);
        }
        let _z = rectangle.max.z;
        let rectangle = Box2 {
            min: rectangle.min.xy(),
            max: rectangle.max.xy(),
        };

        let clipped_rectangle = self.clip_rectangle(rectangle);

        if color.alpha == 0.0 {
            return clipped_rectangle;
        }

        if clipped_rectangle.area() != 0.0 {
            let color = color.to_srgb();

            let (width, height) = rectangle.size().into();
            let min_radius = (width / 2.).min(height / 2.);
            let radius = corner_radius.min(Vec4::fill(min_radius));

            let center_index = self.first_mesh.positions.len() as u32;

            let center = rectangle.center();
            self.push_position(Vec3::new(center.x, center.y, 0.0));

            self.first_mesh.colors.push(color);
            self.first_mesh.texture_coordinates.push(Vec2::ZERO);

            let corner_radius = radius[0];

            self.corner(
                corner_radius,
                center_index,
                Vec3::new(
                    rectangle.min.x + corner_radius,
                    rectangle.min.y + corner_radius,
                    0.0,
                ),
                std::f32::consts::PI * 1.0,
                color,
            );

            self.first_mesh.extend_indices(&[[
                center_index,
                self.first_mesh.positions.len() as u32 - 1,
                self.first_mesh.positions.len() as u32,
            ]]);

            self.corner(
                corner_radius,
                center_index,
                Vec3::new(
                    rectangle.max.x - corner_radius,
                    rectangle.min.y + corner_radius,
                    0.0,
                ),
                std::f32::consts::PI * 1.5,
                color,
            );

            self.first_mesh.extend_indices(&[[
                center_index,
                self.first_mesh.positions.len() as u32 - 1,
                self.first_mesh.positions.len() as u32,
            ]]);

            self.corner(
                corner_radius,
                center_index,
                Vec3::new(
                    rectangle.max.x - corner_radius,
                    rectangle.max.y - corner_radius,
                    0.0,
                ),
                0.0,
                color,
            );
            self.first_mesh.extend_indices(&[[
                center_index,
                self.first_mesh.positions.len() as u32 - 1,
                self.first_mesh.positions.len() as u32,
            ]]);

            self.corner(
                corner_radius,
                center_index,
                Vec3::new(
                    rectangle.min.x + corner_radius,
                    rectangle.max.y - corner_radius,
                    0.0,
                ),
                std::f32::consts::PI * 0.5,
                color,
            );

            self.first_mesh.extend_indices(&[[
                center_index,
                self.first_mesh.positions.len() as u32 - 1,
                center_index + 1,
            ]]);
        }
        clipped_rectangle
    }
}
