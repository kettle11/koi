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
}
pub trait GetEventHandlers<State> {
    fn event_handlers_mut(&mut self) -> &mut EventHandlers<State>;
}

pub struct StandardInput {
    pub text_input_rect: Option<Box2>,
    pub delta_time: f32,
    pub input_events: Vec<kapp_platform_common::Event>,
    pub input_events_handled: Vec<bool>,
    pub view_size: Vec2,
    pub button_clicked: bool,
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
            view_size: Vec2::ZERO,
            button_clicked: false,
        }
    }
}

pub struct EventHandlers<State> {
    click_handlers: Vec<(Box3, Option<Box<dyn Fn(&mut State)>>)>,
}
pub struct StandardContext<State> {
    pub style: StandardStyle,
    pub input: StandardInput,
    pub fonts: Fonts,
    pub event_handlers: EventHandlers<State>,
}

impl<State> StandardContext<State> {
    pub fn new(style: StandardStyle, input: StandardInput, fonts: Fonts) -> Self {
        Self {
            style,
            input,
            fonts,
            event_handlers: EventHandlers {
                click_handlers: Vec::new(),
            },
        }
    }
}

impl<State> GetStandardStyle for StandardContext<State> {
    fn standard_style(&self) -> &StandardStyle {
        self.style.standard_style()
    }

    fn standard_style_mut(&mut self) -> &mut StandardStyle {
        &mut self.style
    }
}
impl<State> GetStandardInput for StandardContext<State> {
    fn standard_input(&self) -> &StandardInput {
        self.input.borrow()
    }

    fn standard_input_mut(&mut self) -> &mut StandardInput {
        &mut self.input
    }
}

impl<State> GetFonts for StandardContext<State> {
    fn get_fonts(&self) -> &Fonts {
        &self.fonts
    }

    fn get_fonts_mut(&mut self) -> &Fonts {
        &mut self.fonts
    }
}
