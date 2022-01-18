use crate::*;

pub fn text<State: GetStandardStyle>(text: impl Into<TextSource<State>>) -> Text<State> {
    Text::new(
        text,
        |state: &State| state.standard_style().primary_font,
        |state: &State| state.standard_style().primary_text_color,
        |state: &State| state.standard_style().primary_text_size,
    )
}

pub fn heading<State: GetStandardStyle>(text: impl Into<TextSource<State>>) -> Text<State> {
    Text::new(
        text,
        |state: &State| state.standard_style().heading_font,
        |state: &State| state.standard_style().primary_text_color,
        |state: &State| state.standard_style().primary_text_size,
    )
}

pub enum TextSource<State> {
    Data(Box<dyn Fn(&State) -> String + Send>),
    String(&'static str),
}

impl<Data> From<&'static str> for TextSource<Data> {
    fn from(s: &'static str) -> Self {
        Self::String(s)
    }
}

impl<Data, F: Fn(&Data) -> String + 'static + Send> From<F> for TextSource<Data> {
    fn from(f: F) -> Self {
        Self::Data(Box::new(f))
    }
}

pub struct Text<State: GetStandardStyle> {
    text_source: TextSource<State>,
    get_font: fn(&State) -> Font,
    get_color: fn(&State) -> Color,
    get_size: fn(&State) -> f32,
    layout: fontdue::layout::Layout,
}

impl<State: GetStandardStyle> Text<State> {
    pub fn new(
        text: impl Into<TextSource<State>>,
        get_font: fn(&State) -> Font,
        get_color: fn(&State) -> Color,
        get_size: fn(&State) -> f32,
    ) -> Self {
        Self {
            text_source: text.into(),
            get_font,
            get_color,
            get_size,
            layout: fontdue::layout::Layout::new(fontdue::layout::CoordinateSystem::PositiveYDown),
        }
    }

    pub fn draw_text_with_color(
        &mut self,
        state: &mut State,
        drawer: &mut Drawer,
        rectangle: Box2,
        color: Color,
    ) {
        let layout = &mut self.layout;

        let font_index = (self.get_font)(state).0;
        let font = &state.standard_style().fonts()[font_index];
        drawer.text(
            font,
            layout,
            rectangle.min,
            color,
            state.standard_style().ui_scale,
        )
    }
}

impl<
        State: GetStandardStyle,
        Constraints: GetStandardConstraints + Default,
        Drawer: GetStandardDrawer,
    > Widget<State, Constraints, Drawer> for Text<State>
{
    fn layout(&mut self, state: &mut State) -> Constraints {
        // This layout should instead be stored in standard state.
        let layout = &mut self.layout;

        // Various alignments could be introduced here.
        layout.reset(&Default::default());

        let font_index = (self.get_font)(state).0;
        let fonts = state.standard_style().fonts();

        let ui_scale = state.standard_style().ui_scale;
        let text_size = (self.get_size)(state) * ui_scale;

        match &self.text_source {
            TextSource::String(s) => {
                layout.append(
                    fonts,
                    &fontdue::layout::TextStyle::new(s, text_size, font_index),
                );
            }
            TextSource::Data(d) => {
                let s = (d)(state);
                layout.append(
                    fonts,
                    &fontdue::layout::TextStyle::new(&s, text_size, font_index),
                );
            }
        };
        let size = layout
            .glyphs()
            .iter()
            .fold(Box2::ZERO, |total_bounds, glyph| {
                total_bounds.join(Box2::new_with_min_corner_and_size(
                    Vec2::new(glyph.x, glyph.y) / ui_scale,
                    Vec2::new(glyph.width as f32, glyph.height as f32) / ui_scale,
                ))
            })
            .size();
        let mut constraints = Constraints::default();
        constraints.standard_mut().set_size(size);
        constraints
    }

    fn draw(&mut self, state: &mut State, drawer: &mut Drawer, constraints: Constraints) {
        let color = (self.get_color)(state);
        self.draw_text_with_color(
            state,
            drawer.standard(),
            constraints.standard().bounds,
            color,
        )
    }
}
