use crate::*;

// Column Widget
pub struct Column<CONTEXT: UIContextTrait> {
    children: Vec<(Vec2, Box<dyn WidgetTrait<CONTEXT>>)>,
}

pub fn column<CONTEXT: UIContextTrait + 'static, const SIZE: usize>(
    children: [Box<dyn WidgetTrait<CONTEXT>>; SIZE],
) -> Box<dyn WidgetTrait<CONTEXT>> {
    let mut size_and_child = Vec::with_capacity(children.len());
    for child in children {
        size_and_child.push((Vec2::ZERO, child))
    }
    Box::new(Column {
        children: size_and_child,
    })
}

impl<CONTEXT: UIContextTrait> WidgetTrait<CONTEXT> for Column<CONTEXT> {
    fn size(
        &mut self,
        context: &mut CONTEXT,
        style: &mut CONTEXT::Style,
        data: &mut CONTEXT::Data,
    ) -> Vec2 {
        let mut total_size = Vec2::ZERO;
        for (size, child) in &mut self.children {
            *size = child.size(context, style, data);
            total_size.x = total_size.x.max(size.x);
            total_size.y += size.y;
        }
        total_size
    }

    fn draw(
        &mut self,
        context: &mut CONTEXT,
        style: &mut CONTEXT::Style,
        data: &mut CONTEXT::Data,
        drawer: &mut Drawer,
        rectangle: Rectangle,
    ) {
        let mut y = rectangle.min.y;
        for (size, child) in &mut self.children {
            let min = Vec2::new(rectangle.min.x, y);
            child.draw(
                context,
                style,
                data,
                drawer,
                Rectangle::new(min, min + *size),
            );
            y += size.y;
        }
    }

    fn event(&mut self, context: &mut CONTEXT, data: &mut CONTEXT::Data, event: &Event) {
        for (_, child) in &mut self.children {
            child.event(context, data, event)
        }
    }
}

// Row widget
pub struct Row<CONTEXT: UIContextTrait> {
    children: Vec<(Vec2, Box<dyn WidgetTrait<CONTEXT>>)>,
}

pub fn row<CONTEXT: UIContextTrait + 'static, const SIZE: usize>(
    children: [Box<dyn WidgetTrait<CONTEXT>>; SIZE],
) -> Box<dyn WidgetTrait<CONTEXT>> {
    let mut size_and_child = Vec::with_capacity(children.len());
    for child in children {
        size_and_child.push((Vec2::ZERO, child))
    }
    Box::new(Row {
        children: size_and_child,
    })
}

impl<CONTEXT: UIContextTrait> WidgetTrait<CONTEXT> for Row<CONTEXT> {
    fn size(
        &mut self,
        context: &mut CONTEXT,
        style: &mut CONTEXT::Style,
        data: &mut CONTEXT::Data,
    ) -> Vec2 {
        let mut total_size = Vec2::ZERO;
        for (size, child) in &mut self.children {
            *size = child.size(context, style, data);
            total_size.y = total_size.y.max(size.y);
            total_size.x += size.x;
        }
        total_size
    }

    fn draw(
        &mut self,
        context: &mut CONTEXT,
        style: &mut CONTEXT::Style,
        data: &mut CONTEXT::Data,
        drawer: &mut Drawer,
        rectangle: Rectangle,
    ) {
        let mut x = rectangle.min.x;
        for (size, child) in &mut self.children {
            let min = Vec2::new(x, rectangle.min.y);
            child.draw(
                context,
                style,
                data,
                drawer,
                Rectangle::new(min, min + *size),
            );
            x += size.x;
        }
    }

    fn event(&mut self, context: &mut CONTEXT, data: &mut CONTEXT::Data, event: &Event) {
        for (_, child) in &mut self.children {
            child.event(context, data, event)
        }
    }
}
