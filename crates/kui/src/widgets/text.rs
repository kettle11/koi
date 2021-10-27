use crate::*;

pub fn text<Style: GetStandardStyleTrait + 'static, Data>(
    text: impl Into<TextSource<Data>>,
) -> Text<Style, Data> {
    Text::new(
        text,
        |style: &Style| style.standard().primary_font,
        |style: &Style| style.standard().primary_text_color,
        |style: &Style| style.standard().primary_text_size,
    )
}

pub fn heading<Style: GetStandardStyleTrait + 'static, Data>(
    text: impl Into<TextSource<Data>>,
) -> Text<Style, Data> {
    Text::new(
        text,
        |style: &Style| style.standard().heading_font,
        |style: &Style| style.standard().primary_text_color,
        |style: &Style| style.standard().primary_text_size,
    )
}

pub enum TextSource<Data> {
    Data(Box<dyn Fn(&mut Data) -> String + Send>),
    String(&'static str),
}

impl<Data> From<&'static str> for TextSource<Data> {
    fn from(s: &'static str) -> Self {
        Self::String(s)
    }
}

impl<Data, F: Fn(&mut Data) -> String + 'static + Send> From<F> for TextSource<Data> {
    fn from(f: F) -> Self {
        Self::Data(Box::new(f))
    }
}

pub struct Text<Style, Data> {
    text_source: TextSource<Data>,
    get_font: fn(&Style) -> Font,
    get_color: fn(&Style) -> Color,
    get_size: fn(&Style) -> f32,
    layout: fontdue::layout::Layout,
}

impl<Style: GetStandardStyleTrait, Data> Text<Style, Data> {
    pub fn new(
        text: impl Into<TextSource<Data>>,
        get_font: fn(&Style) -> Font,
        get_color: fn(&Style) -> Color,
        get_size: fn(&Style) -> f32,
    ) -> Self {
        Self {
            text_source: text.into(),
            get_font,
            get_color,
            get_size,
            layout: fontdue::layout::Layout::new(fontdue::layout::CoordinateSystem::PositiveYDown),
        }
    }

    pub fn draw_with_color(
        &mut self,
        style: &mut Style,
        drawer: &mut Drawer,
        rectangle: Rectangle,
        color: Color,
    ) {
        let layout = &mut self.layout;

        let font_index = (self.get_font)(style).0;
        let font = &style.standard().fonts()[font_index];
        drawer.text(
            font,
            layout,
            rectangle.min,
            color,
            style.standard().ui_scale,
        )
    }
}

impl<Style: 'static, Data: 'static> WidgetTrait<Style, Data> for Text<Style, Data>
where
    Style: GetStandardStyleTrait,
{
    fn size(&mut self, style: &mut Style, data: &mut Data) -> Vec2 {
        let layout = &mut self.layout;

        // Various alignments could be introduced here.
        layout.reset(&Default::default());

        let font_index = (self.get_font)(style).0;
        let fonts = style.standard().fonts();

        let ui_scale = style.standard().ui_scale;
        let text_size = (self.get_size)(style) * ui_scale;

        match &self.text_source {
            TextSource::String(s) => {
                layout.append(
                    fonts,
                    &fontdue::layout::TextStyle::new(s, text_size, font_index),
                );
            }
            TextSource::Data(d) => {
                let s = (d)(data);
                layout.append(
                    fonts,
                    &fontdue::layout::TextStyle::new(&s, text_size, font_index),
                );
            }
        };
        let size = layout
            .glyphs()
            .iter()
            .fold(BoundingBox::ZERO, |total_bounds, glyph| {
                total_bounds.join(BoundingBox::new_with_min_corner_and_size(
                    Vec2::new(glyph.x, glyph.y) / ui_scale,
                    Vec2::new(glyph.width as f32, glyph.height as f32) / ui_scale,
                ))
            })
            .size();
        size
    }

    fn draw(
        &mut self,
        style: &mut Style,
        _data: &mut Data,
        drawer: &mut Drawer,
        rectangle: Rectangle,
    ) {
        let color = (self.get_color)(style);
        self.draw_with_color(style, drawer, rectangle, color)
    }
}
