use crate::*;

pub trait ProduceChildrenTrait<CONTEXT: UIContextTrait>: Send + 'static {
    fn add_children_initial(self, children: &mut Vec<(Vec2, Box<dyn WidgetTrait<CONTEXT>>)>);
    fn add_dynamic_children(
        &mut self,
        data: &mut CONTEXT::Data,
        children: &mut Vec<(Vec2, Box<dyn WidgetTrait<CONTEXT>>)>,
    );
    fn dynamic(&self) -> bool;
}

impl<CONTEXT: UIContextTrait, const SIZE: usize> ProduceChildrenTrait<CONTEXT>
    for [Box<dyn WidgetTrait<CONTEXT>>; SIZE]
{
    fn add_children_initial(self, children: &mut Vec<(Vec2, Box<dyn WidgetTrait<CONTEXT>>)>) {
        children.reserve(self.len());
        for child in self {
            children.push((Vec2::ZERO, child));
        }
    }
    fn add_dynamic_children(
        &mut self,
        _data: &mut CONTEXT::Data,
        _children: &mut Vec<(Vec2, Box<dyn WidgetTrait<CONTEXT>>)>,
    ) {
    }
    fn dynamic(&self) -> bool {
        false
    }
}

/// Used to add children to a widget that has multiple children.
pub struct ChildAdder<'a, CONTEXT> {
    children: &'a mut Vec<(Vec2, Box<dyn WidgetTrait<CONTEXT>>)>,
}

impl<CONTEXT> ChildAdder<'_, CONTEXT> {
    pub fn add_child(&mut self, child: Box<dyn WidgetTrait<CONTEXT>>) {
        self.children.push((Vec2::ZERO, child))
    }
}

impl<CONTEXT: UIContextTrait, F: Fn(&mut CONTEXT::Data, ChildAdder<CONTEXT>) + Send + 'static>
    ProduceChildrenTrait<CONTEXT> for F
{
    fn add_children_initial(self, _children: &mut Vec<(Vec2, Box<dyn WidgetTrait<CONTEXT>>)>) {}
    fn add_dynamic_children(
        &mut self,
        data: &mut CONTEXT::Data,
        children: &mut Vec<(Vec2, Box<dyn WidgetTrait<CONTEXT>>)>,
    ) {
        children.clear();
        let child_adder = ChildAdder { children };
        let i = (self)(data, child_adder);
    }
    fn dynamic(&self) -> bool {
        true
    }
}

// Column Widget
pub struct Column<CONTEXT: UIContextTrait, ChildProducer: ProduceChildrenTrait<CONTEXT>> {
    child_producer: Option<ChildProducer>,
    children: Vec<(Vec2, Box<dyn WidgetTrait<CONTEXT>>)>,
}

pub fn column<CONTEXT: UIContextTrait + 'static, ChildProducer: ProduceChildrenTrait<CONTEXT>>(
    children: ChildProducer,
) -> Box<dyn WidgetTrait<CONTEXT>> {
    let mut size_and_child = Vec::new();
    if !children.dynamic() {
        children.add_children_initial(&mut size_and_child);
        Box::new(Column::<CONTEXT, ChildProducer> {
            child_producer: None,
            children: size_and_child,
        })
    } else {
        Box::new(Column {
            child_producer: Some(children),
            children: size_and_child,
        })
    }
}

impl<CONTEXT: UIContextTrait, ChildProducer: ProduceChildrenTrait<CONTEXT>> WidgetTrait<CONTEXT>
    for Column<CONTEXT, ChildProducer>
{
    fn size(
        &mut self,
        context: &mut CONTEXT,
        style: &mut CONTEXT::Style,
        data: &mut CONTEXT::Data,
    ) -> Vec2 {
        if let Some(child_producer) = &mut self.child_producer {
            child_producer.add_dynamic_children(data, &mut self.children)
        }

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
