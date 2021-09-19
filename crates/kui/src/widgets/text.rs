use crate::*;

pub fn text<CONTEXT: UIContextTrait>(
    text: impl Into<TextSource<CONTEXT::Data>>,
) -> Box<dyn WidgetTrait<CONTEXT>>
where
    CONTEXT::Style: GetStandardStyleTrait,
{
    Box::new(Text::new(
        text,
        |style: &CONTEXT::Style| style.standard().primary_font,
        |style: &CONTEXT::Style| style.standard().primary_text_color,
    ))
}

pub fn heading<CONTEXT: UIContextTrait>(
    text: impl Into<TextSource<CONTEXT::Data>>,
) -> Box<dyn WidgetTrait<CONTEXT>>
where
    CONTEXT::Style: GetStandardStyleTrait,
{
    Box::new(Text::new(
        text,
        |style: &CONTEXT::Style| style.standard().heading_font,
        |style: &CONTEXT::Style| style.standard().primary_text_color,
    ))
}

pub enum TextSource<DATA> {
    Data(Box<dyn Fn(&DATA) -> String + Send>),
    String(String),
}

impl<DATA> From<String> for TextSource<DATA> {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl<DATA> From<&str> for TextSource<DATA> {
    fn from(s: &str) -> Self {
        Self::String(s.into())
    }
}

impl<DATA, F: Fn(&DATA) -> String + 'static + Send> From<F> for TextSource<DATA> {
    fn from(f: F) -> Self {
        Self::Data(Box::new(f))
    }
}

pub struct Text<DATA, STYLE> {
    text_source: TextSource<DATA>,
    get_font: fn(&STYLE) -> Font,
    get_text_color: fn(&STYLE) -> Color,
    layout: fontdue::layout::Layout,
}

impl<DATA, STYLE> Text<DATA, STYLE> {
    pub fn new(
        text: impl Into<TextSource<DATA>>,
        get_font: fn(&STYLE) -> Font,
        get_text_color: fn(&STYLE) -> Color,
    ) -> Self {
        Self {
            text_source: text.into(),
            get_font,
            get_text_color,
            layout: fontdue::layout::Layout::new(fontdue::layout::CoordinateSystem::PositiveYDown),
        }
    }
}

impl<CONTEXT: UIContextTrait> WidgetTrait<CONTEXT> for Text<CONTEXT::Data, CONTEXT::Style>
where
    CONTEXT::Style: GetStandardStyleTrait,
{
    fn size(
        &mut self,
        _context: &mut CONTEXT,
        style: &mut CONTEXT::Style,
        data: &mut CONTEXT::Data,
    ) -> Vec2 {
        let layout = &mut self.layout;

        // Various alignments could be introduced here.
        layout.reset(&Default::default());

        let font_index = (self.get_font)(style).0;
        let fonts = style.standard().fonts();
        match &self.text_source {
            TextSource::String(s) => {
                layout.append(fonts, &fontdue::layout::TextStyle::new(s, 40., font_index));
            }
            TextSource::Data(d) => {
                let s = (d)(data);
                layout.append(fonts, &fontdue::layout::TextStyle::new(&s, 40., font_index));
            }
        };
        let size = layout
            .glyphs()
            .iter()
            .fold(BoundingBox::ZERO, |total_bounds, glyph| {
                total_bounds.join(BoundingBox::new_with_min_corner_and_size(
                    Vec2::new(glyph.x, glyph.y),
                    Vec2::new(glyph.width as f32, glyph.height as f32),
                ))
            })
            .size();
        size
    }

    fn draw(
        &mut self,
        _context: &mut CONTEXT,
        style: &mut CONTEXT::Style,
        _data: &mut CONTEXT::Data,
        drawer: &mut Drawer,
        rectangle: Rectangle,
    ) {
        let layout = &mut self.layout;

        let font_index = (self.get_font)(style).0;
        let font = &style.standard().fonts()[font_index];
        drawer.text(&font, layout, rectangle.min, (self.get_text_color)(style))
    }
}
