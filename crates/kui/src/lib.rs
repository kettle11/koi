use std::borrow::Borrow;

use kcolor::*;
use kmath::*;

mod drawer;
pub use drawer::Drawer;

mod style;
pub use style::*;

mod texture_atlas;

mod widgets2;
pub use widgets2::*;

mod fonts;
pub use fonts::*;

#[derive(Clone, Copy)]
pub struct MinAndMaxSize {
    pub min: Vec3,
    pub max: Vec3,
}

pub trait Widget<Data, Context> {
    #[allow(unused)]
    fn update(&mut self, data: &mut Data, context: &mut Context) {}
    /// Perform any layout work required and let the parent widget know the Constraints this child requires.
    /// Note that while 'data' is mutable it should not be edited during `layout`.
    fn layout(
        &mut self,
        data: &mut Data,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3;

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

    /// This can be used in scenarios where the Context internals have been cloned
    /// This is mostly a workaround, there's almost certainly a better design.
    fn try_standard_input_mut(&mut self) -> Option<&mut StandardInput>;
}

pub struct StandardInput {
    pub text_input_rect: Option<Box2>,
    pub delta_time: f32,
    pub input_events: Vec<kapp_platform_common::Event>,
    pub input_events_handled: Vec<bool>,
}

impl StandardInput {
    pub fn input_events_iter(
        &mut self,
    ) -> impl Iterator<Item = (&mut bool, kapp_platform_common::Event)> {
        self.input_events_handled
            .iter_mut()
            .zip(self.input_events.iter().cloned())
    }
}
impl Default for StandardInput {
    fn default() -> Self {
        Self {
            text_input_rect: None,
            delta_time: 0.0,
            input_events: Vec::new(),
            input_events_handled: Vec::new(),
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
    fn try_standard_input_mut(&mut self) -> Option<&mut StandardInput> {
        Some(self)
    }
}

pub struct StandardContext<Style, Input> {
    pub style: std::rc::Rc<Style>,
    pub input: std::rc::Rc<Input>,
    pub fonts: std::rc::Rc<Fonts>,
}

impl<Style, Input> Clone for StandardContext<Style, Input> {
    fn clone(&self) -> Self {
        Self {
            style: self.style.clone(),
            input: self.input.clone(),
            fonts: self.fonts.clone(),
        }
    }
}

impl<Style, Input> StandardContext<Style, Input> {
    pub fn new(style: Style, input: Input, fonts: Fonts) -> Self {
        Self {
            style: std::rc::Rc::new(style),
            input: std::rc::Rc::new(input),
            fonts: std::rc::Rc::new(fonts),
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
    fn try_standard_input_mut(&mut self) -> Option<&mut StandardInput> {
        std::rc::Rc::get_mut(&mut self.input).map(|i| i.standard_input_mut())
    }
}

impl<Style, Input: GetStandardInput> GetFonts for StandardContext<Style, Input> {
    fn get_fonts(&self) -> &Fonts {
        self.fonts.borrow()
    }
    fn get_fonts_mut(&mut self) -> &Fonts {
        std::rc::Rc::get_mut(&mut self.fonts).unwrap()
    }
}
