use crate::*;

pub trait ProduceChildrenTrait<Data, Child>: 'static + Send {
    fn add_children_initial(self, children: &mut Vec<(Vec2, Child)>);
    fn add_dynamic_children(&mut self, data: &mut Data, children: &mut Vec<(Vec2, Child)>);
    fn dynamic(&self) -> bool;
}

impl<Data, Child: 'static + Send, const SIZE: usize> ProduceChildrenTrait<Data, Child>
    for [Child; SIZE]
{
    fn add_children_initial(self, children: &mut Vec<(Vec2, Child)>) {
        children.reserve(self.len());
        for child in self {
            children.push((Vec2::ZERO, child));
        }
    }
    fn add_dynamic_children(&mut self, _data: &mut Data, _children: &mut Vec<(Vec2, Child)>) {}
    fn dynamic(&self) -> bool {
        false
    }
}

/// Used to add children to a widget that has multiple children.
pub struct ChildAdder<'a, Child> {
    children: &'a mut Vec<(Vec2, Child)>,
}

impl<Child> ChildAdder<'_, Child> {
    pub fn add_child(&mut self, child: Child) {
        self.children.push((Vec2::ZERO, child))
    }
}

impl<Data, Child, F: Fn(&mut Data, ChildAdder<Child>) + Send + 'static>
    ProduceChildrenTrait<Data, Child> for F
{
    fn add_children_initial(self, _children: &mut Vec<(Vec2, Child)>) {}
    fn add_dynamic_children(&mut self, data: &mut Data, children: &mut Vec<(Vec2, Child)>) {
        children.clear();
        let child_adder = ChildAdder { children };
        (self)(data, child_adder);
    }
    fn dynamic(&self) -> bool {
        true
    }
}

// Column Widget
pub struct Column<
    Style,
    Data,
    Child: WidgetTrait<Style, Data>,
    ChildProducer: ProduceChildrenTrait<Data, Child>,
> {
    child_producer: Option<ChildProducer>,
    children: Vec<(Vec2, Child)>,
    phantom: std::marker::PhantomData<fn() -> (Style, Data)>,
}

pub fn column<
    Style,
    Data,
    Child: WidgetTrait<Style, Data> + 'static,
    ChildProducer: ProduceChildrenTrait<Data, Child>,
>(
    children: ChildProducer,
) -> Column<Style, Data, Child, ChildProducer> {
    let mut size_and_child = Vec::new();
    if !children.dynamic() {
        children.add_children_initial(&mut size_and_child);
        Column {
            child_producer: None,
            children: size_and_child,
            phantom: std::marker::PhantomData,
        }
    } else {
        Column {
            child_producer: Some(children),
            children: size_and_child,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<
        Style: 'static,
        Data: 'static,
        Child: WidgetTrait<Style, Data>,
        ChildProducer: ProduceChildrenTrait<Data, Child>,
    > WidgetTrait<Style, Data> for Column<Style, Data, Child, ChildProducer>
{
    fn size(&mut self, style: &mut Style, data: &mut Data) -> Vec2 {
        if let Some(child_producer) = &mut self.child_producer {
            child_producer.add_dynamic_children(data, &mut self.children)
        }

        let mut total_size = Vec2::ZERO;
        for (size, child) in &mut self.children {
            *size = child.size(style, data);
            total_size.x = total_size.x.max(size.x);
            total_size.y += size.y;
        }
        total_size
    }

    fn draw(
        &mut self,
        style: &mut Style,
        data: &mut Data,
        drawer: &mut Drawer,
        rectangle: Rectangle,
    ) {
        let mut y = rectangle.min.y;
        for (size, child) in &mut self.children {
            let min = Vec2::new(rectangle.min.x, y);
            child.draw(style, data, drawer, Rectangle::new(min, min + *size));
            y += size.y;
        }
    }

    fn event(&mut self, data: &mut Data, event: &Event) {
        for (_, child) in &mut self.children {
            child.event(data, event)
        }
    }
}

// Column Widget
pub struct Stack<
    Style,
    Data,
    Child: WidgetTrait<Style, Data>,
    ChildProducer: ProduceChildrenTrait<Data, Child>,
> {
    child_producer: Option<ChildProducer>,
    children: Vec<(Vec2, Child)>,
    phantom: std::marker::PhantomData<fn() -> (Style, Data)>,
}

pub fn stack<
    Style,
    Data,
    Child: WidgetTrait<Style, Data> + 'static,
    ChildProducer: ProduceChildrenTrait<Data, Child>,
>(
    children: ChildProducer,
) -> Column<Style, Data, Child, ChildProducer> {
    let mut size_and_child = Vec::new();
    if !children.dynamic() {
        children.add_children_initial(&mut size_and_child);
        Column {
            child_producer: None,
            children: size_and_child,
            phantom: std::marker::PhantomData,
        }
    } else {
        Column {
            child_producer: Some(children),
            children: size_and_child,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<
        Style: 'static,
        Data: 'static,
        Child: WidgetTrait<Style, Data>,
        ChildProducer: ProduceChildrenTrait<Data, Child>,
    > WidgetTrait<Style, Data> for Stack<Style, Data, Child, ChildProducer>
{
    fn size(&mut self, style: &mut Style, data: &mut Data) -> Vec2 {
        if let Some(child_producer) = &mut self.child_producer {
            child_producer.add_dynamic_children(data, &mut self.children)
        }

        let mut total_size = Vec2::ZERO;
        for (size, child) in &mut self.children {
            *size = child.size(style, data);
            total_size.x = total_size.x.max(size.x);
            total_size.y += total_size.y.max(size.y);
        }
        total_size
    }

    fn draw(
        &mut self,
        style: &mut Style,
        data: &mut Data,
        drawer: &mut Drawer,
        rectangle: Rectangle,
    ) {
        for (size, child) in &mut self.children {
            child.draw(style, data, drawer, rectangle);
        }
    }

    fn event(&mut self, data: &mut Data, event: &Event) {
        for (_, child) in &mut self.children {
            child.event(data, event)
        }
    }
}

/*
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
        style: &mut Style,
        data: &mut Data,
    ) -> Vec2 {
        let mut total_size = Vec2::ZERO;
        for (size, child) in &mut self.children {
            *size = child.size( style, data);
            total_size.y = total_size.y.max(size.y);
            total_size.x += size.x;
        }
        total_size
    }

    fn draw(
        &mut self,
        style: &mut Style,
        data: &mut Data,
        drawer: &mut Drawer,
        rectangle: Rectangle,
    ) {
        let mut x = rectangle.min.x;
        for (size, child) in &mut self.children {
            let min = Vec2::new(x, rectangle.min.y);
            child.draw(

                style,
                data,
                drawer,
                Rectangle::new(min, min + *size),
            );
            x += size.x;
        }
    }

    fn event(&mut self,  data: &mut Data, event: &Event) {
        for (_, child) in &mut self.children {
            child.event( data, event)
        }
    }
}
*/
