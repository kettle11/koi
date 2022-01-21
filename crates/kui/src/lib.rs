use kcolor::*;
use kmath::*;

mod drawer;
pub use drawer::Drawer;

mod style;
pub use style::*;

mod texture_atlas;

mod widgets2;
pub use widgets2::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct Font(usize);

pub struct MinAndMaxSize {
    pub min: Vec3,
    pub max: Vec3,
}

pub trait Widget<Data, Context> {
    #[allow(unused)]
    fn update(&mut self, data: &mut Data, context: &mut Context) {}
    /// Perform any layout work required and let the parent widget know the Constraints this child requires.
    /// Note that while 'data' is mutable it should not be edited during `layout`.
    fn layout(&mut self, data: &mut Data, context: &mut Context) -> Vec3;

    /// Note that while 'data' is mutable it should not be edited during `draw`.
    fn draw(&mut self, data: &mut Data, context: &mut Context, drawer: &mut Drawer, bounds: Box3);
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
    pub style: std::rc::Rc<Style>,
    pub input: std::rc::Rc<Input>,
}

impl<Style, Input> Clone for StandardContext<Style, Input> {
    fn clone(&self) -> Self {
        Self {
            style: self.style.clone(),
            input: self.input.clone(),
        }
    }
}

impl<Style, Input> StandardContext<Style, Input> {
    pub fn new(style: Style, input: Input) -> Self {
        Self {
            style: std::rc::Rc::new(style),
            input: std::rc::Rc::new(input),
        }
    }
}

impl<Style: GetStandardStyle, Input> GetStandardStyle for StandardContext<Style, Input> {
    fn standard_style(&self) -> &StandardStyle {
        self.style.standard_style()
    }

    fn standard_style_mut(&mut self) -> &mut StandardStyle {
        std::rc::Rc::get_mut(&mut self.style)
            .unwrap()
            .standard_style_mut()
    }
}
impl<Style, Input: GetStandardInput> GetStandardInput for StandardContext<Style, Input> {
    fn standard_input(&self) -> &StandardInput {
        self.input.standard_input()
    }

    fn standard_input_mut(&mut self) -> &mut StandardInput {
        std::rc::Rc::get_mut(&mut self.input)
            .unwrap()
            .standard_input_mut()
    }
}
