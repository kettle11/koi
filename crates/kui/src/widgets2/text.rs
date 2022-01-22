use crate::*;

pub fn text<State, Context: GetStandardStyle + GetFonts>(
    text: impl Into<TextSource<State>>,
) -> Text<State, Context> {
    Text::new(
        text,
        |context: &Context| context.standard_style().primary_font,
        |context: &Context| context.standard_style().primary_text_color,
        |context: &Context| context.standard_style().primary_text_size,
    )
}

pub fn heading<State, Context: GetStandardStyle + GetFonts>(
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
    MutString(fn(&mut State) -> &mut String),
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

impl<Data> From<fn(&mut Data) -> &mut String> for TextSource<Data> {
    fn from(f: fn(&mut Data) -> &mut String) -> Self {
        Self::MutString(f)
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

pub struct Text<State, Context: GetStandardStyle + GetFonts> {
    text_source: TextSource<State>,
    get_font: fn(&Context) -> Font,
    get_color: fn(&Context) -> Color,
    get_size: fn(&Context) -> f32,
    layout: fontdue::layout::Layout,
}

impl<State, Context: GetStandardStyle + GetFonts> Text<State, Context> {
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
        let ui_scale = context.standard_style().ui_scale;
        let layout = &mut self.layout;

        let font_index = (self.get_font)(context).0;
        let fonts = context.get_fonts().fonts();

        let font = &fonts[font_index];
        drawer.text(font, layout, rectangle.min, color, ui_scale)
    }

    pub fn get_character_bounds(
        &mut self,
        context: &mut Context,
        offset: Vec3,
        index: usize,
    ) -> Box2 {
        let ui_scale = context.standard_style().ui_scale;
        let glyph = self.layout.glyphs()[index];
        Drawer::glyph_position(offset, ui_scale, &glyph)
    }

    pub fn get_character_count(&mut self) -> usize {
        self.layout.glyphs().len()
    }

    pub fn get_line_height(&mut self, context: &mut Context) -> f32 {
        let ui_scale = context.standard_style().ui_scale;
        let text_size = (self.get_size)(context) * ui_scale;

        let font_index = (self.get_font)(context).0;
        let fonts = context.get_fonts().fonts();
        let font = &fonts[font_index];
        let metrics = font.horizontal_line_metrics(text_size).unwrap();
        metrics.new_line_size / ui_scale
    }
}

impl<State, Context: GetStandardStyle + GetFonts> Widget<State, Context> for Text<State, Context> {
    fn layout(&mut self, state: &mut State, context: &mut Context) -> Vec3 {
        let ui_scale = context.standard_style().ui_scale;

        // This layout should instead be stored in standard state.
        let layout = &mut self.layout;

        // Various alignments could be introduced here.
        layout.reset(&Default::default());

        let font_index = (self.get_font)(context).0;
        let fonts = context.get_fonts().fonts();

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
            TextSource::MutString(d) => {
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
        let mut size = layout
            .glyphs()
            .iter()
            .fold(Box2::ZERO, |total_bounds, glyph| {
                total_bounds.join(Box2::new_with_min_corner_and_size(
                    Vec2::new(glyph.x, glyph.y) / ui_scale,
                    Vec2::new(glyph.width as f32, glyph.height as f32) / ui_scale,
                ))
            })
            .size();

        // Prevent text from shrinking its requested size when there's no text.
        size.y = size.y.max(self.get_line_height(context));
        size.extend(0.1)
    }

    fn draw(
        &mut self,
        state: &mut State,
        context: &mut Context,
        drawer: &mut Drawer,
        bounds: Box3,
    ) {
        let color = (self.get_color)(context);
        self.draw_text_with_color(state, context, drawer.standard(), bounds, color)
    }
}
