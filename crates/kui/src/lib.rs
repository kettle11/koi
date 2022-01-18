use kcolor::*;
use kmath::*;

//mod widgets;
//pub use widgets::*;

mod drawer;
pub use drawer::Drawer;

mod style;
pub use style::*;

mod texture_atlas;

mod widgets2;
pub use widgets2::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct Font(usize);

pub trait Widget<State, Constraints, Drawer> {
    fn update(&mut self, _state: &mut State) {}
    /// Perform any layout work required and let the parent widget know the Constraints this child requires.
    /// Note that while 'state' is mutable it should not be edited during `layout`.
    fn layout(&mut self, state: &mut State) -> Constraints;

    /// Note that while 'state' is mutable it should not be edited during `draw`.
    fn draw(&mut self, _state: &mut State, _drawer: &mut Drawer, _constraints: Constraints) {}
    fn update_layout_draw(
        &mut self,
        state: &mut State,
        drawer: &mut Drawer,
        initial_constraints: Constraints,
    ) {
        self.update(state);
        self.layout(state);
        self.draw(state, drawer, initial_constraints);
    }
}

pub trait GetStandardDrawer {
    fn standard(&mut self) -> &mut Drawer;
}

impl GetStandardDrawer for Drawer {
    fn standard(&mut self) -> &mut Drawer {
        self
    }
}

pub trait GetStandardConstraints {
    fn standard(&self) -> &StandardConstraints;
    fn standard_mut(&mut self) -> &mut StandardConstraints;
}

impl GetStandardConstraints for StandardConstraints {
    fn standard(&self) -> &StandardConstraints {
        self
    }
    fn standard_mut(&mut self) -> &mut StandardConstraints {
        self
    }
}

#[derive(Clone, Copy)]
pub struct StandardConstraints {
    pub bounds: Box2,
}

impl StandardConstraints {
    pub fn set_size(&mut self, size: Vec2) {
        self.bounds.max = self.bounds.min + size;
    }
}
impl Default for StandardConstraints {
    fn default() -> Self {
        Self { bounds: Box2::ZERO }
    }
}

pub trait GetStandardStyle {
    fn standard_style(&self) -> &StandardStyle;
    fn standard_style_mut(&mut self) -> &mut StandardStyle;
}
