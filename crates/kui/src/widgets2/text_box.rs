use crate::*;

pub fn text_field<Data, Context: GetStandardStyle + GetFonts + GetStandardInput>(
    get_text: fn(&mut Data) -> &mut String,
) -> impl Widget<Data, Context> {
    fit(stack((
        outlined_rounded_fill(
            |c: &Context| c.standard_style().primary_variant_color,
            |c| c.standard_style().primary_color,
            |c| c.standard_style().rounding,
        ),
        padding(|c: &Context| c.standard_style().padding, text_box(get_text)),
    )))
}
pub fn text_box<Data, Context: GetStandardStyle + GetFonts + GetStandardInput>(
    get_text: fn(&mut Data) -> &mut String,
) -> impl Widget<Data, Context> {
    TextBox {
        get_text,
        cursor_offset_from_end: 0,
        child_text: text(get_text),
        cursor_animation: 0.0,
        cursor_on: false,
    }
}
pub struct TextBox<Data, Context: GetStandardStyle + GetFonts> {
    get_text: fn(&mut Data) -> &mut String,
    /// The offset from the end of the string in numbers of characters
    cursor_offset_from_end: usize,
    child_text: Text<Data, Context>,
    cursor_animation: f32,
    cursor_on: bool,
}

impl<Data, Context: GetStandardStyle + GetFonts + GetStandardInput> Widget<Data, Context>
    for TextBox<Data, Context>
{
    fn update(&mut self, data: &mut Data, context: &mut Context) {
        context.standard_input_mut().text_input_rect = Some(Box2::ZERO);
        let string = (self.get_text)(data);
        let character_count = self.child_text.get_character_count();

        // This isn't very efficient for large strings.
        // This also isn't implemented correctly if the size of the String changes.
        let cursor_position = string
            .char_indices()
            .skip(character_count - self.cursor_offset_from_end)
            .next();
        for &char in context.standard_input().characters_input.iter() {
            if let Some((cursor_position, _)) = cursor_position {
                string.insert(cursor_position, char)
            } else {
                string.push(char);
            }
        }
        for &key in context.standard_input().keys_pressed.iter() {
            match key {
                kapp_platform_common::Key::Backspace => {
                    if let Some((cursor_position, _)) = cursor_position {
                        if cursor_position != 0 {
                            string.remove(cursor_position - 1);
                        }
                    } else {
                        string.pop();
                    }
                }
                kapp_platform_common::Key::Left => {
                    self.cursor_offset_from_end += 1;
                }
                kapp_platform_common::Key::Right => {
                    if self.cursor_offset_from_end > 0 {
                        self.cursor_offset_from_end -= 1;
                    }
                }
                _ => {}
            }
        }

        if self.cursor_offset_from_end > character_count {
            self.cursor_offset_from_end = character_count;
        }
    }
    fn layout(&mut self, data: &mut Data, context: &mut Context) -> Vec3 {
        self.child_text.layout(data, context)
    }
    fn draw(&mut self, data: &mut Data, context: &mut Context, drawer: &mut Drawer, bounds: Box3) {
        self.child_text.draw(data, context, drawer, bounds);

        self.cursor_animation += context.standard_input().delta_time;

        let cursor_blink_speed = 0.6;
        if self.cursor_animation > cursor_blink_speed {
            self.cursor_on = !self.cursor_on;
            self.cursor_animation -= cursor_blink_speed;
        }
        if self.cursor_on {
            let glyph_count = self.child_text.get_character_count();

            let cursor_left = if (glyph_count - self.cursor_offset_from_end) != 0 {
                self.child_text
                    .get_character_bounds(
                        context,
                        bounds.min,
                        glyph_count - self.cursor_offset_from_end - 1,
                    )
                    .max
                    .x
            } else {
                if glyph_count != 0 {
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

            let line_height = self.child_text.get_line_height(context);

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
