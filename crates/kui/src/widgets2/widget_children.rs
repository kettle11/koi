use crate::*;

/// A trait used to define things that produce children for widgets that can accept multiple children.
pub trait WidgetChildren<Data, Context>: for<'a> GetConstraintsIter<'a> {
    fn create_children_and_layout(
        &mut self,
        state: &mut Data,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    );
    fn draw<F: FnMut(&Vec3) -> Box3>(
        &mut self,
        state: &mut Data,
        context: &mut Context,
        drawer: &mut Drawer,
        f: F,
    );
    fn len(&self) -> usize;
}

pub trait GetConstraintsIter<'a> {
    type ConstraintsIter: Iterator<Item = &'a Vec3>;
    fn constraints_iter(&'a self) -> Self::ConstraintsIter;
}

pub trait IntoWidgetChildren<Data, Context> {
    type WidgetChildren: WidgetChildren<Data, Context>;
    fn into_widget_children(self) -> Self::WidgetChildren;
}

impl<Data, Context, Child: Widget<Data, Context>> IntoWidgetChildren<Data, Context>
    for ChildForEach<Data, Context, Child>
{
    type WidgetChildren = Self;
    fn into_widget_children(self) -> Self::WidgetChildren {
        self
    }
}

pub struct ChildCreator<'a, Child> {
    current_child_index: usize,
    children: &'a mut Vec<Child>,
}

impl<'a, Child> ChildCreator<'a, Child> {
    pub fn add_child(&mut self, f: impl Fn() -> Child) {
        if self.current_child_index >= self.children.len() {
            self.children.push(f())
        }
        self.current_child_index += 1;
    }
}

pub struct ChildForEach<Data, Context, Child: Widget<Data, Context>> {
    constraints: Vec<Vec3>,
    children: Vec<Child>,
    create_children: fn(&mut Data, &mut ChildCreator<Child>),
    phantom: std::marker::PhantomData<fn() -> (Context, Drawer)>,
}

pub fn for_each<Data, Context, Child: Widget<Data, Context>>(
    create_children: fn(&mut Data, &mut ChildCreator<Child>),
) -> ChildForEach<Data, Context, Child> {
    ChildForEach {
        constraints: Vec::new(),
        children: Vec::new(),
        create_children,
        phantom: std::marker::PhantomData,
    }
}

impl<Data, Context, Child: Widget<Data, Context>> WidgetChildren<Data, Context>
    for ChildForEach<Data, Context, Child>
{
    fn create_children_and_layout(
        &mut self,
        state: &mut Data,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) {
        let Self {
            children,
            create_children,
            ..
        } = self;
        let mut child_creator = ChildCreator {
            current_child_index: 0,
            children,
        };
        (create_children)(state, &mut child_creator);
        self.constraints.resize(self.children.len(), Vec3::ZERO);

        for (child, constraint) in self.children.iter_mut().zip(self.constraints.iter_mut()) {
            *constraint = (child).layout(state, context, min_and_max_size);
        }
    }

    fn draw<F: FnMut(&Vec3) -> Box3>(
        &mut self,
        state: &mut Data,
        context: &mut Context,
        drawer: &mut Drawer,
        mut f: F,
    ) {
        for (child, child_constraints) in self.children.iter_mut().zip(self.constraints.iter()) {
            let child_constraints = f(child_constraints);
            child.draw(state, context, drawer, child_constraints);
        }
    }
    fn len(&self) -> usize {
        self.children.len()
    }
}

impl<'a, Data, Context, Child: Widget<Data, Context>> GetConstraintsIter<'a>
    for ChildForEach<Data, Context, Child>
{
    type ConstraintsIter = std::slice::Iter<'a, Vec3>;
    fn constraints_iter(&'a self) -> Self::ConstraintsIter {
        self.constraints.iter()
    }
}

pub struct TupleChildren<T, const CHILD_COUNT: usize> {
    constraints: [Vec3; CHILD_COUNT],
    children: T,
}

impl<'a, T, const CHILD_COUNT: usize> GetConstraintsIter<'a> for TupleChildren<T, CHILD_COUNT> {
    type ConstraintsIter = std::slice::Iter<'a, Vec3>;
    fn constraints_iter(&'a self) -> Self::ConstraintsIter {
        self.constraints.iter()
    }
}

// This macro is used to generate child impls for a bunch of tuples.
// This lets code like `column((widget_a, widget_b, widget_c))` work.
macro_rules! tuple_impls {
    ( $count: tt, $( ($index: tt, $tuple:ident) ),* ) => {
            impl<
                Data,
                Context,
                $( $tuple: Widget<Data, Context>,)*
            > IntoWidgetChildren<Data, Context> for ($( $tuple,)*)
        {
            type WidgetChildren = TupleChildren<($( $tuple,)*), $count>;
            fn into_widget_children(self) -> Self::WidgetChildren {
                TupleChildren {
                    constraints: [Vec3::ZERO; $count],
                    children: self,
                }
            }
        }

        impl<
            Data,
            Context,
            $( $tuple: Widget<Data, Context>,)*
         > WidgetChildren<Data, Context>
            for TupleChildren<($( $tuple,)*), $count>
        {

            #[allow(unused)]
            fn create_children_and_layout(&mut self, state: &mut Data, context: &mut Context, min_and_max_size: MinAndMaxSize) {
                $(self.constraints[$index] = self.children.$index.layout(state, context, min_and_max_size);)*
            }

            #[allow(unused)]
            fn draw<FUNCTION: FnMut(&Vec3) -> Box3>(
                &mut self,
                state: &mut Data,
                context: &mut Context,
                drawer: &mut Drawer,
                mut f: FUNCTION,
            ) {
                $(self.children.$index.draw(state, context, drawer, f(&self.constraints[$index]));)*
            }

            fn len(&self) -> usize {
                $count
            }
        }
    }
}

tuple_impls! {0,}
tuple_impls! { 1, (0, A) }
tuple_impls! { 2, (0, A), (1, B) }
tuple_impls! { 3, (0, A), (1, B), (2, C) }
tuple_impls! { 4, (0, A), (1, B), (2, C), (3, D)}
tuple_impls! { 5, (0, A), (1, B), (2, C), (3, D), (4, E)}
tuple_impls! { 6, (0, A), (1, B), (2, C), (3, D), (4, E), (5, F)}
tuple_impls! { 7, (0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G)}
tuple_impls! { 8, (0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H)}
tuple_impls! { 9, (0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I)}
tuple_impls! { 10, (0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J)}
tuple_impls! { 11, (0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J), (10, K)}
