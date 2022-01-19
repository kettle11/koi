use crate::*;

pub fn text<State, Context: GetStandardStyle>(
    text: impl Into<TextSource<State>>,
) -> Text<State, Context> {
    Text::new(
        text,
        |context: &Context| context.standard_style().primary_font,
        |context: &Context| context.standard_style().primary_text_color,
        |context: &Context| context.standard_style().primary_text_size,
    )
}

pub fn heading<State, Context: GetStandardStyle>(
    text: impl Into<TextSource<State>>,
) -> Text<State, Context> {
    Text::new(
        text,
        |context: &Context| context.standard_style().heading_font,
        |context: &Context| context.standard_style().primary_text_color,
        |context: &Context| context.standard_style().heading_text_size,
    )
}

pub enum TextSource<State> {
    Data(Box<dyn Fn(&State) -> String + Send + 'static>),
    DataDyn(fn(&State) -> &str),
    String(&'static str),
}

impl<Data> From<&'static str> for TextSource<Data> {
    fn from(s: &'static str) -> Self {
        Self::String(s)
    }
}

pub struct StrSource<Data>(pub fn(&Data) -> &str);

impl<Data> From<fn(&Data) -> &str> for TextSource<Data> {
    fn from(f: fn(&Data) -> &str) -> Self {
        Self::DataDyn(f)
    }
}

impl<Data> From<StrSource<Data>> for TextSource<Data> {
    fn from(f: StrSource<Data>) -> Self {
        Self::DataDyn(f.0)
    }
}
impl<Data, F: Fn(&Data) -> String + Send + 'static> From<F> for TextSource<Data> {
    fn from(f: F) -> Self {
        Self::Data(Box::new(f))
    }
}

pub struct Text<State, Context: GetStandardStyle> {
    text_source: TextSource<State>,
    get_font: fn(&Context) -> Font,
    get_color: fn(&Context) -> Color,
    get_size: fn(&Context) -> f32,
    layout: fontdue::layout::Layout,
}

impl<State, Context: GetStandardStyle> Text<State, Context> {
    pub fn new(
        text: impl Into<TextSource<State>>,
        get_font: fn(&Context) -> Font,
        get_color: fn(&Context) -> Color,
        get_size: fn(&Context) -> f32,
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
        _state: &mut State,
        context: &mut Context,
        drawer: &mut Drawer,
        rectangle: Box3,
        color: Color,
    ) {
        let layout = &mut self.layout;

        let font_index = (self.get_font)(context).0;
        let font = &context.standard_style().fonts()[font_index];
        drawer.text(
            font,
            layout,
            rectangle.min,
            color,
            context.standard_style().ui_scale,
        )
    }
}

impl<
        State,
        Context: GetStandardStyle,
        Constraints: GetStandardConstraints + Default,
        Drawer: GetStandardDrawer,
    > Widget<State, Context, Constraints, Drawer> for Text<State, Context>
{
    fn layout(&mut self, state: &mut State, context: &mut Context) -> Constraints {
        // This layout should instead be stored in standard state.
        let layout = &mut self.layout;

        // Various alignments could be introduced here.
        layout.reset(&Default::default());

        let font_index = (self.get_font)(context).0;
        let fonts = context.standard_style().fonts();

        let ui_scale = context.standard_style().ui_scale;
        let text_size = (self.get_size)(context) * ui_scale;

        match &self.text_source {
            TextSource::String(s) => {
                layout.append(
                    fonts,
                    &fontdue::layout::TextStyle::new(s, text_size, font_index),
                );
            }
            TextSource::DataDyn(d) => {
                let s = (d)(state);
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
        constraints.standard_mut().set_size(size.extend(0.1));
        constraints
    }

    fn draw(
        &mut self,
        state: &mut State,
        context: &mut Context,
        drawer: &mut Drawer,
        constraints: Constraints,
    ) {
        let color = (self.get_color)(context);
        self.draw_text_with_color(
            state,
            context,
            drawer.standard(),
            constraints.standard().bounds,
            color,
        )
    }
}
