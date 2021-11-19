use crate::*;

pub struct ChildrenCreator<'a, ChildData, Child> {
    index: usize,
    children: &'a mut Vec<Child>,
    for_each: &'a mut dyn for<'b> FnMut(&'b mut ChildData, &'b mut Child),
}

impl<'a, ChildData, Child> ChildrenCreator<'a, ChildData, Child> {
    pub fn child<'b>(&'b mut self, child_data: &'b mut ChildData, create_child: fn() -> Child) {
        if self.index <= self.children.len() {
            let child = create_child();
            self.children.push(child);
        }
        self.index += 1;
        (self.for_each)(child_data, &mut self.children[self.index - 1])
    }
}

/// Column

pub fn column<Style, Data, ChildData, Child>(
    create_children: fn(&mut Data, ChildrenCreator<ChildData, Child>),
) -> Column<Style, Data, ChildData, Child> {
    Column {
        create_children,
        children: Vec::new(),
        sizes: Vec::new(),
        phantom: std::marker::PhantomData,
    }
}

pub struct Column<Style, Data, ChildData, Child> {
    create_children: fn(&mut Data, ChildrenCreator<ChildData, Child>),
    children: Vec<Child>,
    sizes: Vec<Vec2>,
    #[allow(clippy::type_complexity)]
    phantom: std::marker::PhantomData<fn() -> (Style, Data, ChildData)>,
}

impl<
        Style: Send + 'static + GetStandardStyleTrait,
        Data: Send + 'static,
        ChildData: 'static,
        Child: WidgetTrait<Style, ChildData>,
    > WidgetTrait<Style, Data> for Column<Style, Data, ChildData, Child>
{
    fn size(&mut self, style: &mut Style, data: &mut Data) -> Vec2 {
        let Self {
            create_children,
            children,
            sizes,
            ..
        } = self;

        let mut total_size = Vec2::ZERO;
        let spacing = style.standard().column_spacing;

        sizes.clear();
        let children_creator_size = ChildrenCreator {
            index: 0,
            children,
            for_each: &mut |child_data, child| {
                let size = child.size(style, child_data);
                sizes.push(size);
                total_size.x = total_size.x.max(size.x);
                total_size.y += size.y;
                total_size.y += spacing;
            },
        };
        (create_children)(data, children_creator_size);
        total_size.y -= spacing;
        total_size
    }

    fn draw(&mut self, style: &mut Style, data: &mut Data, drawer: &mut Drawer, rectangle: Box2) {
        let Self {
            create_children,
            children,
            sizes,
            ..
        } = self;

        let mut index = 0;
        let mut y = rectangle.min.y;
        let spacing = style.standard().row_spacing;

        let children_creator_draw = ChildrenCreator {
            index: 0,
            children,
            for_each: &mut |child_data, child| {
                let size = sizes[index];
                let rectangle = Box2::new(
                    Vec2::new(rectangle.min.x, y),
                    Vec2::new(rectangle.max.x, y + size.y),
                );
                y += size.y + spacing;
                child.draw(style, child_data, drawer, rectangle);
                index += 1;
            },
        };
        (create_children)(data, children_creator_draw)
    }

    fn event(&mut self, data: &mut Data, event: &Event) -> bool {
        let Self {
            create_children,
            children,
            sizes: _,
            ..
        } = self;

        let mut handled_event = false;
        let children_creator_draw = ChildrenCreator {
            index: 0,
            children,
            for_each: &mut |child_data, child| {
                handled_event |= child.event(child_data, event);
            },
        };
        (create_children)(data, children_creator_draw);
        handled_event
    }
}

/// Row

pub fn row<Style, Data, ChildData, Child>(
    create_children: fn(&mut Data, ChildrenCreator<ChildData, Child>),
) -> Row<Style, Data, ChildData, Child> {
    Row {
        create_children,
        children: Vec::new(),
        sizes: Vec::new(),
        phantom: std::marker::PhantomData,
    }
}

pub struct Row<Style, Data, ChildData, Child> {
    create_children: fn(&mut Data, ChildrenCreator<ChildData, Child>),
    children: Vec<Child>,
    sizes: Vec<Vec2>,
    #[allow(clippy::type_complexity)]
    phantom: std::marker::PhantomData<fn() -> (Style, Data, ChildData)>,
}

impl<
        Style: Send + 'static + GetStandardStyleTrait,
        Data: Send + 'static,
        ChildData: 'static,
        Child: WidgetTrait<Style, ChildData>,
    > WidgetTrait<Style, Data> for Row<Style, Data, ChildData, Child>
{
    fn size(&mut self, style: &mut Style, data: &mut Data) -> Vec2 {
        let Self {
            create_children,
            children,
            sizes,
            ..
        } = self;

        let mut total_size = Vec2::ZERO;
        sizes.clear();
        let spacing = style.standard().row_spacing;

        let children_creator_size = ChildrenCreator {
            index: 0,
            children,
            for_each: &mut |child_data, child| {
                let size = child.size(style, child_data);
                sizes.push(size);
                total_size.x += size.x;
                total_size.y = total_size.y.max(size.y);
                total_size.x += spacing;
            },
        };
        (create_children)(data, children_creator_size);
        total_size.x -= spacing;
        total_size
    }

    fn draw(&mut self, style: &mut Style, data: &mut Data, drawer: &mut Drawer, rectangle: Box2) {
        let Self {
            create_children,
            children,
            sizes,
            ..
        } = self;

        let mut index = 0;
        let mut x = rectangle.min.x;
        let spacing = style.standard().row_spacing;

        let children_creator_draw = ChildrenCreator {
            index: 0,
            children,
            for_each: &mut |child_data, child| {
                let size = sizes[index];
                let rectangle = Box2::new(
                    Vec2::new(x, rectangle.min.y),
                    Vec2::new(x + size.x, rectangle.max.y),
                );
                x += size.x + spacing;
                child.draw(style, child_data, drawer, rectangle);
                index += 1;
            },
        };
        (create_children)(data, children_creator_draw)
    }

    fn event(&mut self, data: &mut Data, event: &Event) -> bool {
        let Self {
            create_children,
            children,
            sizes: _,
            ..
        } = self;

        let mut handled_event = false;
        let children_creator_draw = ChildrenCreator {
            index: 0,
            children,
            for_each: &mut |child_data, child| {
                handled_event |= child.event(child_data, event);
            },
        };
        (create_children)(data, children_creator_draw);
        handled_event
    }
}
