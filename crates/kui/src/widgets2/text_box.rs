use crate::*;

pub fn text_field<
    Data,
    Context: GetStandardStyle + GetFonts + GetStandardInput + GetEventHandlers<Data>,
    ExtraState,
>(
    get_text: fn(&mut Data) -> &mut String,
) -> impl Widget<Data, Context, ExtraState> {
    fit(stack((
        outlined_rounded_fill(
            |_, _, c: &Context| c.standard_style().primary_variant_color,
            |_, _, c| c.standard_style().primary_color,
            |_, c| c.standard_style().rounding,
        ),
        padding(|c: &Context| c.standard_style().padding, text_box(get_text)),
    )))
}
pub fn text_box<Data, Context: GetStandardStyle + GetFonts + GetStandardInput, ExtraState>(
    get_text: fn(&mut Data) -> &mut String,
) -> impl Widget<Data, Context, ExtraState> {
    TextBox {
        _get_text: get_text,
        cursor_offset_from_end: 0,
        child_text: text(get_text),
        cursor_animation: 0.0,
        cursor_on: false,
        selected_area: None,
    }
}
pub struct TextBox<Data, Context: GetStandardStyle + GetFonts, ExtraState> {
    _get_text: fn(&mut Data) -> &mut String,
    /// The offset from the end of the string in numbers of characters.
    /// This points to a character. The cursor should go *before* the character.
    cursor_offset_from_end: usize,
    child_text: Text<Data, Context, ExtraState>,
    cursor_animation: f32,
    cursor_on: bool,
    selected_area: Option<(usize, usize)>,
}
impl<Data, Context: GetStandardStyle + GetFonts + GetStandardInput, ExtraState>
    TextBox<Data, Context, ExtraState>
{
    pub fn select_all(&mut self) {
        let character_count = self.child_text.get_character_count();
        self.reset_cursor_animation();
        self.cursor_offset_from_end = 0;
        self.selected_area = Some((0, character_count))
    }

    pub fn reset_cursor_animation(&mut self) {
        self.cursor_on = true;
        self.cursor_animation = 0.0;
    }
}

impl<Data, Context: GetStandardStyle + GetFonts + GetStandardInput, ExtraState>
    Widget<Data, Context, ExtraState> for TextBox<Data, Context, ExtraState>
{
    /*
    fn update(&mut self, data: &mut Data, context: &mut Context) {
        context.standard_input_mut().text_input_rect = Some(Box2::ZERO);
        let string = (self.get_text)(data);
        let mut character_count = self.child_text.get_character_count();

        // This isn't very efficient for large strings.

        let mut char_indices_iter = string
            .char_indices()
            .skip((character_count - self.cursor_offset_from_end).saturating_sub(1));
        let (remove_index, remove_range) = char_indices_iter
            .next()
            .map_or((0, 0), |(i, c)| (i, c.len_utf8()));
        let mut edit_index = char_indices_iter.next().map_or(character_count, |(i, _)| i);

        let at_start = character_count - self.cursor_offset_from_end == 0;

        for (_handled, event) in context.standard_input_mut().input_events_iter() {
            match event {
                kapp_platform_common::Event::CharacterReceived { character, .. } => {
                    self.reset_cursor_animation();
                    string.insert(edit_index, character);
                    edit_index += character.len_utf8();
                }
                kapp_platform_common::Event::KeyDown { key, .. } => match key {
                    kapp_platform_common::Key::Backspace => {
                        if !at_start {
                            self.cursor_on = true;
                            self.cursor_animation = 0.0;
                            string.replace_range(remove_index..remove_index + remove_range, &"");
                            character_count = character_count.saturating_sub(1);
                        }
                    }
                    kapp_platform_common::Key::Left => {
                        self.selected_area = None;
                        self.reset_cursor_animation();
                        self.cursor_offset_from_end += 1;
                    }
                    kapp_platform_common::Key::Right => {
                        self.selected_area = None;
                        self.reset_cursor_animation();
                        self.cursor_offset_from_end = self.cursor_offset_from_end.saturating_sub(1);
                    }
                    kapp_platform_common::Key::Meta => {
                        self.select_all();
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if self.cursor_offset_from_end > character_count {
            self.cursor_offset_from_end = character_count;
        }

        if let Some((start, end)) = &mut self.selected_area {
            *start = (*start).min(character_count);
            *end = (*end).min(character_count);
        }
        if character_count == 0 {
            self.selected_area = None;
        }
    }
    */

    fn layout(
        &mut self,
        data: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        self.child_text
            .layout(data, extra_state, context, min_and_max_size)
    }
    fn draw(
        &mut self,
        data: &mut Data,
        extra_state: &mut ExtraState,
        context: &mut Context,
        drawer: &mut Drawer,
        bounds: Box3,
    ) {
        let line_height = self.child_text.get_line_height(data, extra_state, context);

        // Draw selected area highlight. Notably it's drawn before the child to ensure it's drawn underneath.
        if let Some((start, end)) = self.selected_area {
            let start_bounds = self
                .child_text
                .get_character_bounds(context, bounds.min, start)
                .min
                .x;
            let end_bounds = bounds.min.x
                + self.child_text.get_glyph_advance_width_position(
                    data,
                    extra_state,
                    context,
                    end - 1,
                );
            drawer.rectangle(
                Box3 {
                    min: Vec3::new(start_bounds, bounds.min.y, bounds.min.z),
                    max: Vec3::new(end_bounds, bounds.min.y + line_height, bounds.min.z),
                },
                context.standard_style().disabled_color,
            );
        }

        self.child_text
            .draw(data, extra_state, context, drawer, bounds);

        self.cursor_animation += context.standard_input().delta_time;

        let cursor_blink_speed = 0.6;
        if self.cursor_animation > cursor_blink_speed {
            self.cursor_on = !self.cursor_on;
            self.cursor_animation -= cursor_blink_speed;
        }
        if self.cursor_on {
            let glyph_count = self.child_text.get_character_count();

            let cursor_left = if (glyph_count - self.cursor_offset_from_end) != 0 {
                // If this isn't before the first character.
                bounds.min.x
                    + self.child_text.get_glyph_advance_width_position(
                        data,
                        extra_state,
                        context,
                        glyph_count - self.cursor_offset_from_end - 1,
                    )
            } else {
                // If this is before the first character
                if glyph_count != 0 {
                    // If there is a next character.
                    self.child_text
                        .get_character_bounds(
                            context,
                            bounds.min,
                            glyph_count - self.cursor_offset_from_end,
                        )
                        .min
                        .x
                        - 1.0
                } else {
                    bounds.min.x
                }
            };

            drawer.rectangle(
                Box3 {
                    min: Vec3::new(cursor_left, bounds.min.y, bounds.min.z),
                    max: Vec3::new(cursor_left + 1.0, bounds.min.y + line_height, bounds.min.z),
                },
                Color::WHITE,
            );
        }
    }
}
