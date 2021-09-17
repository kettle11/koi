use crate::*;
use kmath::BoundingBox;

#[derive(Component, Clone, Debug)]
pub struct Sprite {
    pub texture_handle: Handle<Texture>,
    /// The a rectangle specified in percentage of the texture, not pixels!
    pub sprite_source_bounds: BoundingBox<f32, 2>,
}

impl Sprite {
    /// sprite_source_bounds is a rectangle specified in percentage of the texture, not pixels!
    pub fn new(texture_handle: Handle<Texture>, sprite_source_bounds: BoundingBox<f32, 2>) -> Self {
        Self {
            texture_handle,
            sprite_source_bounds,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SpriteMap {
    texture_handle: Handle<Texture>,
    scale: Vec2,
    padding_scale: Vec2,
}

impl SpriteMap {
    pub fn new(
        texture_handle: Handle<Texture>,
        tile_size: usize,
        padding: usize,
        width: usize,
        height: usize,
    ) -> Self {
        let scale = Vec2::fill(tile_size as f32 + padding as f32)
            .div_by_component(Vec2::new(width as f32, height as f32));
        let padding_scale =
            Vec2::fill(padding as f32).div_by_component(Vec2::new(width as f32, height as f32));

        Self {
            texture_handle,
            scale,
            padding_scale,
        }
    }

    pub fn get_sprite(&self, x: usize, y: usize) -> Sprite {
        let xy = Vec2::new(x as f32, y as f32);
        Sprite {
            texture_handle: self.texture_handle.clone(),
            sprite_source_bounds: BoundingBox::new_with_min_corner_and_size(
                xy.mul_by_component(self.scale),
                self.scale - self.padding_scale,
            ),
        }
    }
}
