use crate::*;

pub fn text<State, Context: GetStandardStyle + GetFonts>(
    text: impl Into<TextSource<State>>,
) -> Text<State, Context> {
    Text::new(
        text,
        |_, context: &Context| context.standard_style().primary_font,
        |_, context: &Context| context.standard_style().primary_text_color,
        |_, context: &Context| context.standard_style().primary_text_size,
    )
}

pub fn heading<State, Context: GetStandardStyle + GetFonts>(
    text: impl Into<TextSource<State>>,
) -> Text<State, Context> {
    Text::new(
        text,
        |_, context: &Context| context.standard_style().heading_font,
        |_, context: &Context| context.standard_style().primary_text_color,
        |_, context: &Context| context.standard_style().heading_text_size,
    )
}

pub enum TextSource<State> {
    Data(Box<dyn Fn(&mut State) -> String + Send + 'static>),
    MutString(fn(&mut State) -> &mut String),
    DataDyn(fn(&mut State) -> &str),
    String(&'static str),
}

impl<Data> From<&'static str> for TextSource<Data> {
    fn from(s: &'static str) -> Self {
        Self::String(s)
    }
}

pub struct StrSource<Data>(pub fn(&mut Data) -> &str);

impl<Data> From<fn(&mut Data) -> &str> for TextSource<Data> {
    fn from(f: fn(&mut Data) -> &str) -> Self {
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
impl<Data, F: Fn(&mut Data) -> String + Send + 'static> From<F> for TextSource<Data> {
    fn from(f: F) -> Self {
        Self::Data(Box::new(f))
    }
}

pub struct Text<State, Context: GetStandardStyle + GetFonts> {
    text_source: TextSource<State>,
    get_font: fn(&mut State, &Context) -> Font,
    get_color: fn(&mut State, &Context) -> Color,
    get_size: fn(&mut State, &Context) -> f32,
    layout: fontdue::layout::Layout,
}

impl<State, Context: GetStandardStyle + GetFonts> Text<State, Context> {
    pub fn with_color(self, get_color: fn(&mut State, &Context) -> Color) -> Self {
        Self { get_color, ..self }
    }

    pub fn with_font(self, get_font: fn(&mut State, &Context) -> Font) -> Self {
        Self { get_font, ..self }
    }

    pub fn with_size(self, get_size: fn(&mut State, &Context) -> f32) -> Self {
        Self { get_size, ..self }
    }
}

impl<State, Context: GetStandardStyle + GetFonts> Text<State, Context> {
    pub fn new(
        text: impl Into<TextSource<State>>,
        get_font: fn(&mut State, &Context) -> Font,
        get_color: fn(&mut State, &Context) -> Color,
        get_size: fn(&mut State, &Context) -> f32,
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
        context: &mut Context,
        drawer: &mut Drawer,
        rectangle: Box3,
        color: Color,
    ) {
        let ui_scale = context.standard_style().ui_scale;
        let layout = &mut self.layout;

        let font_index = (self.get_font)(state, context).0;
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

    pub fn get_glyph_advance_width_position(
        &mut self,
        state: &mut State,
        context: &mut Context,
        index: usize,
    ) -> f32 {
        let glyph_position = self.layout.glyphs()[index];

        let font_index = (self.get_font)(state, context).0;
        let fonts = context.get_fonts().fonts();
        let font = &fonts[font_index];

        let ui_scale = context.standard_style().ui_scale;
        let text_size = (self.get_size)(state, context) * ui_scale;
        (font
            .metrics_indexed(glyph_position.key.glyph_index, text_size)
            .advance_width as f32
            + glyph_position.x)
            / ui_scale
    }

    pub fn get_character_count(&mut self) -> usize {
        self.layout.glyphs().len()
    }

    pub fn get_line_height(&mut self, state: &mut State, context: &mut Context) -> f32 {
        let ui_scale = context.standard_style().ui_scale;
        let text_size = (self.get_size)(state, context) * ui_scale;

        let font_index = (self.get_font)(state, context).0;
        let fonts = context.get_fonts().fonts();
        let font = &fonts[font_index];
        let metrics = font.horizontal_line_metrics(text_size).unwrap();
        metrics.new_line_size / ui_scale
    }
}

impl<State, Context: GetStandardStyle + GetFonts> Widget<State, Context> for Text<State, Context> {
    fn layout(
        &mut self,
        state: &mut State,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        let ui_scale = context.standard_style().ui_scale;

        // This layout should instead be stored in standard state.
        let layout = &mut self.layout;

        layout.reset(&fontdue::layout::LayoutSettings {
            max_width: Some(min_and_max_size.max.x * ui_scale),
            max_height: Some(min_and_max_size.max.y * ui_scale),
            ..Default::default()
        });

        let font_index = (self.get_font)(state, context).0;
        let fonts = context.get_fonts().fonts();

        let text_size = (self.get_size)(state, context) * ui_scale;

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
        size.y = size.y.max(self.get_line_height(state, context));
        size.extend(0.1)
    }

    fn draw(
        &mut self,
        state: &mut State,
        context: &mut Context,
        drawer: &mut Drawer,
        bounds: Box3,
    ) {
        let color = (self.get_color)(state, context);
        self.draw_text_with_color(state, context, drawer.standard(), bounds, color)
    }
}
