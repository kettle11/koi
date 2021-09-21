use kmath::*;

use crate::texture_atlas::*;
use crate::*;

pub struct Drawer {
    pub positions: Vec<Vec3>,
    pub texture_coordinates: Vec<Vec2>,
    pub colors: Vec<Vec4>,
    pub indices: Vec<[u32; 3]>,
    pub(crate) view_width: f32,
    pub(crate) view_height: f32,
    pub texture_atlas: TextureAtlas,
    pub clipping_mask: Rectangle,
}

impl Drawer {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            texture_coordinates: Vec::new(),
            colors: Vec::new(),
            indices: Vec::new(),
            view_width: 100.,
            view_height: 100.,
            texture_atlas: TextureAtlas::new(1024),
            clipping_mask: BoundingBox::new(-Vec2::MAX, Vec2::MAX),
        }
    }

    pub fn set_view_width_height(&mut self, width: f32, height: f32) {
        self.view_width = width;
        self.view_height = height;
    }

    pub fn reset(&mut self) {
        self.positions.clear();
        self.texture_coordinates.clear();
        self.colors.clear();
        self.indices.clear();
        self.clipping_mask = BoundingBox::new(-Vec2::MAX, Vec2::MAX);
    }

    pub fn text(
        &mut self,
        fontdue_font: &fontdue::Font,
        layout: &mut fontdue::layout::Layout,
        offset: Vec2,
        color: Color,
        scale: f32,
    ) {
        for c in layout.glyphs() {
            let atlas_rectangle = self
                .texture_atlas
                .get_character(fontdue_font, c.key)
                .unwrap();

            let atlas_rectangle = Rectangle::new_with_min_corner_and_size(
                Vec2::new(
                    atlas_rectangle.x as f32 / self.texture_atlas.width as f32,
                    atlas_rectangle.y as f32 / self.texture_atlas.height as f32,
                ),
                Vec2::new(
                    atlas_rectangle.width as f32 / self.texture_atlas.width as f32,
                    atlas_rectangle.height as f32 / self.texture_atlas.height as f32,
                ),
            );

            let x = c.x / scale + offset.x;
            let y = c.y / scale + offset.y;

            let width = c.width as f32 / scale;
            let height = c.height as f32 / scale;

            let offset = self.positions.len() as u32;
            self.positions.extend_from_slice(&[
                self.position_to_gl(Vec2::new(x, y).extend(0.0)),
                self.position_to_gl(Vec2::new(x + width, y).extend(0.0)),
                self.position_to_gl(Vec2::new(x + width, y + height).extend(0.0)),
                self.position_to_gl(Vec2::new(x, y + height).extend(0.0)),
            ]);

            self.texture_coordinates.extend_from_slice(&[
                Vec2::new(atlas_rectangle.min.x, atlas_rectangle.min.y),
                Vec2::new(atlas_rectangle.max.x, atlas_rectangle.min.y),
                Vec2::new(atlas_rectangle.max.x, atlas_rectangle.max.y),
                Vec2::new(atlas_rectangle.min.x, atlas_rectangle.max.y),
            ]);

            let color = color.to_linear_srgb();
            self.colors.extend_from_slice(&[color, color, color, color]);
            self.extend_indices(&[
                [offset, offset + 1, offset + 2],
                [offset, offset + 2, offset + 3],
            ]);
        }
    }

    fn clip_rectangle(&mut self, rectangle: Rectangle) -> Rectangle {
        rectangle.intersection(self.clipping_mask)
    }

    /// Returns the rectangle that will actually be displayed.
    pub fn rectangle(&mut self, rectangle: Rectangle, color: Color) -> Rectangle {
        let rectangle = self.clip_rectangle(rectangle);
        if rectangle.area() != 0.0 {
            let color = color.to_linear_srgb();
            let (width, height) = rectangle.size().xy().into();
            let (x, y) = rectangle.min.into();

            let offset = self.positions.len() as u32;
            self.positions.extend_from_slice(&[
                self.position_to_gl(Vec3::new(x, y, 0.0)),
                self.position_to_gl(Vec3::new(x + width, y, 0.0)),
                self.position_to_gl(Vec3::new(x + width, y + height, 0.0)),
                self.position_to_gl(Vec3::new(x, y + height, 0.0)),
            ]);
            self.texture_coordinates.extend_from_slice(&[
                Vec2::ZERO,
                Vec2::ZERO,
                Vec2::ZERO,
                Vec2::ZERO,
            ]);

            //   let current_color = Vec4::new(1.0, 1.0, 1.0, 1.0);
            self.colors.extend_from_slice(&[color, color, color, color]);
            self.extend_indices(&[
                [offset, offset + 1, offset + 2],
                [offset, offset + 2, offset + 3],
            ]);
        }
        rectangle
    }

    // Flips indices for OpenGL backend
    fn extend_indices(&mut self, indices: &[[u32; 3]]) {
        self.indices
            .extend(indices.iter().map(|v| [v[2], v[1], v[0]]))
    }

    fn position_to_gl(&self, mut position: Vec3) -> Vec3 {
        position.x = self
            .clipping_mask
            .min
            .x
            .max(position.x)
            .min(self.clipping_mask.max.x);
        position.y = self
            .clipping_mask
            .min
            .y
            .max(position.y)
            .min(self.clipping_mask.max.y);

        let x = (position.x / self.view_width) * 2.0 - 1.0;
        let y = (position.y / self.view_height) * -2.0 + 1.0;
        Vec3::new(x, y, 0.0)
    }

    fn push_position(&mut self, position: Vec3) {
        self.positions.push(self.position_to_gl(position));
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
            let len = self.positions.len() as u32;
            if i != 0 {
                self.extend_indices(&[[center_index, len - 1, len]]);
            }
            let (sin, cos) = angle.sin_cos();
            let position = corner_center + Vec3::new(cos, sin, 0.0) * radius;

            self.push_position(position);
            self.colors.push(color);

            self.texture_coordinates.push(Vec2::ZERO);
            angle += step_amount;
        }
    }

    /// Returns the rectangle that will actually be displayed.
    pub fn rounded_rectangle(
        &mut self,
        rectangle: Rectangle,
        corner_radius: Vec4,
        color: Color,
    ) -> Rectangle {
        let clipped_rectangle = self.clip_rectangle(rectangle);
        if clipped_rectangle.area() != 0.0 {
            if corner_radius == Vec4::fill(0.0) {
                self.rectangle(rectangle, color);
                return clipped_rectangle;
            }

            let color = color.to_linear_srgb();

            let (width, height) = rectangle.size().into();
            let min_radius = (width / 2.).min(height / 2.);
            let radius = corner_radius.min(Vec4::fill(min_radius));

            let center_index = self.positions.len() as u32;

            let center = rectangle.center();
            self.push_position(Vec3::new(center.x, center.y, 0.0));

            self.colors.push(color);
            self.texture_coordinates.push(Vec2::ZERO);

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

            self.extend_indices(&[[
                center_index,
                self.positions.len() as u32 - 1,
                self.positions.len() as u32,
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

            self.extend_indices(&[[
                center_index,
                self.positions.len() as u32 - 1,
                self.positions.len() as u32,
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
            self.extend_indices(&[[
                center_index,
                self.positions.len() as u32 - 1,
                self.positions.len() as u32,
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

            self.extend_indices(&[[
                center_index,
                self.positions.len() as u32 - 1,
                center_index + 1,
            ]]);
        }
        clipped_rectangle
    }
}
