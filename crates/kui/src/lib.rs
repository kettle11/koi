use kcolor::*;
use kmath::*;

mod widgets;
pub use widgets::*;

mod drawer;
pub use drawer::Drawer;

mod style;
pub use style::*;

mod texture_atlas;

pub type Rectangle = BoundingBox<f32, 2>;

pub(crate) use kapp_platform_common::Event;

pub trait UIContextTrait: 'static {
    type Data: 'static;
    type Style: GetStandardStyleTrait + 'static;
}

#[derive(Copy, Clone, Debug)]
pub struct Font(usize);
impl Default for Font {
    fn default() -> Self {
        Self(0)
    }
}
