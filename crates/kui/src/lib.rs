use kcolor::*;
use kmath::*;

mod widgets;
pub use widgets::*;

mod drawer;
pub use drawer::Drawer;

mod style;
pub use style::*;

mod texture_atlas;

pub(crate) use kapp_platform_common::Event;

pub trait UIContextTrait: 'static {
    type Data: 'static;
    type Style: GetStandardStyleTrait + 'static;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Font(usize);
