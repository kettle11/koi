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

pub trait Widget<State, Context, Constraints, Drawer> {
    fn update(&mut self, _state: &mut State, _context: &mut Context) {}
    /// Perform any layout work required and let the parent widget know the Constraints this child requires.
    /// Note that while 'state' is mutable it should not be edited during `layout`.
    fn layout(&mut self, state: &mut State, context: &mut Context) -> Constraints;

    /// Note that while 'state' is mutable it should not be edited during `draw`.
    fn draw(
        &mut self,
        state: &mut State,
        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Constraints,
    );
    fn update_layout_draw(
        &mut self,
        state: &mut State,
        context: &mut Context,
        drawer: &mut Drawer,
        initial_constraints: Constraints,
    ) {
        self.update(state, context);
        self.layout(state, context);
        self.draw(state, context, drawer, initial_constraints);
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
    pub bounds: Box3,
}

impl StandardConstraints {
    pub fn set_size(&mut self, size: Vec3) {
        self.bounds.max = self.bounds.min + size;
    }
}
impl Default for StandardConstraints {
    fn default() -> Self {
        Self { bounds: Box3::ZERO }
    }
}

pub trait GetStandardStyle {
    fn standard_style(&self) -> &StandardStyle;
    fn standard_style_mut(&mut self) -> &mut StandardStyle;
}

pub trait GetStandardInput {
    fn standard_input(&self) -> &StandardInput;
    fn standard_input_mut(&mut self) -> &mut StandardInput;
}

pub struct StandardInput {
    pub pointer_position: Vec2,
    pub pointer_down: bool,
}

impl Default for StandardInput {
    fn default() -> Self {
        Self {
            pointer_position: Vec2::ZERO,
            pointer_down: false,
        }
    }
}

impl GetStandardInput for StandardInput {
    fn standard_input(&self) -> &StandardInput {
        self
    }
    fn standard_input_mut(&mut self) -> &mut StandardInput {
        self
    }
}

pub struct StandardContext<Style, Input> {
    pub style: Style,
    pub input: Input,
}

impl<Style: GetStandardStyle, Input> GetStandardStyle for StandardContext<Style, Input> {
    fn standard_style(&self) -> &StandardStyle {
        self.style.standard_style()
    }

    fn standard_style_mut(&mut self) -> &mut StandardStyle {
        self.style.standard_style_mut()
    }
}
impl<Style, Input: GetStandardInput> GetStandardInput for StandardContext<Style, Input> {
    fn standard_input(&self) -> &StandardInput {
        self.input.standard_input()
    }

    fn standard_input_mut(&mut self) -> &mut StandardInput {
        self.input.standard_input_mut()
    }
}
