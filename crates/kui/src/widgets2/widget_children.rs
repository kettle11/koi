use crate::*;

/// A trait used to define things that produce children for widgets that can accept multiple children.
pub trait WidgetChildren<State, Context, Constraints, Drawer>:
    for<'a> GetConstraintsIter<'a, Constraints>
{
    fn update(&mut self, state: &mut State, context: &mut Context);
    fn create_children_and_layout(&mut self, state: &mut State, context: &mut Context);
    fn draw<F: FnMut(&Constraints) -> Constraints>(
        &mut self,
        state: &mut State,
        context: &mut Context,
        drawer: &mut Drawer,
        f: F,
    );
}

pub trait GetConstraintsIter<'a, Constraints: 'a> {
    type ConstraintsIter: Iterator<Item = &'a Constraints>;
    fn constraints_iter(&'a self) -> Self::ConstraintsIter;
}

pub trait IntoWidgetChildren<State, Context, Constraints, Drawer> {
    type WidgetChildren: WidgetChildren<State, Context, Constraints, Drawer>;
    fn into_widget_children(self) -> Self::WidgetChildren;
}

impl<
        State,
        Context,
        Constraints: 'static + Default,
        Drawer,
        ChildState,
        Child: Widget<ChildState, Context, Constraints, Drawer>,
    > IntoWidgetChildren<State, Context, Constraints, Drawer>
    for ChildForEach<State, Context, Constraints, Drawer, ChildState, Child>
{
    type WidgetChildren = Self;
    fn into_widget_children(self) -> Self::WidgetChildren {
        self
    }
}

pub struct ChildForEach<
    State,
    Context,
    Constraints: 'static,
    Drawer,
    ChildState,
    Child: Widget<ChildState, Context, Constraints, Drawer>,
> {
    constraints: Vec<Constraints>,
    children: Vec<Child>,
    call_per_child: fn(&mut State, &mut dyn FnMut(&mut ChildState)),
    create_child: fn() -> Child,
    phantom: std::marker::PhantomData<fn() -> (Context, Drawer)>,
}

pub fn for_each<
    State,
    Context,
    Constraints: 'static,
    Drawer,
    ChildState,
    Child: Widget<ChildState, Context, Constraints, Drawer>,
>(
    call_per_child: fn(&mut State, &mut dyn FnMut(&mut ChildState)),
    create_child: fn() -> Child,
) -> ChildForEach<State, Context, Constraints, Drawer, ChildState, Child> {
    ChildForEach {
        constraints: Vec::new(),
        children: Vec::new(),
        call_per_child,
        create_child,
        phantom: std::marker::PhantomData,
    }
}

impl<
        State,
        Context,
        Constraints: Default,
        Drawer,
        ChildState,
        Child: Widget<ChildState, Context, Constraints, Drawer>,
    > WidgetChildren<State, Context, Constraints, Drawer>
    for ChildForEach<State, Context, Constraints, Drawer, ChildState, Child>
{
    fn update(&mut self, state: &mut State, context: &mut Context) {
        let mut i = 0;
        (self.call_per_child)(state, &mut |child_state| {
            if let Some(child) = self.children.get_mut(i) {
                child.update(child_state, context);
            }
            i += 1;
        });
    }

    fn create_children_and_layout(&mut self, state: &mut State, context: &mut Context) {
        let mut i = 0;
        (self.call_per_child)(state, &mut |child_state| {
            if i >= self.children.len() {
                self.children.push((self.create_child)());
                self.constraints.push(Constraints::default())
            }
            self.constraints[i] = (self.children[i]).layout(child_state, context);
            i += 1;
        });
    }

    fn draw<F: FnMut(&Constraints) -> Constraints>(
        &mut self,
        state: &mut State,
        context: &mut Context,
        drawer: &mut Drawer,
        mut f: F,
    ) {
        let mut i = 0;
        (self.call_per_child)(state, &mut |child_state| {
            // Pass in already calculated child constraints to get final draw constraints.
            let child_constraints = f(&self.constraints[i]);
            self.children[i].draw(child_state, context, drawer, child_constraints);
            i += 1;
        });
    }
}

impl<
        'a,
        State,
        Context,
        Constraints: 'a,
        Drawer,
        ChildState,
        Child: Widget<ChildState, Context, Constraints, Drawer>,
    > GetConstraintsIter<'a, Constraints>
    for ChildForEach<State, Context, Constraints, Drawer, ChildState, Child>
{
    type ConstraintsIter = std::slice::Iter<'a, Constraints>;
    fn constraints_iter(&'a self) -> Self::ConstraintsIter {
        self.constraints.iter()
    }
}

pub struct TupleChildren<Constraints, T, const CHILD_COUNT: usize> {
    constraints: [Constraints; CHILD_COUNT],
    children: T,
}

impl<'a, Constraints: 'a, T, const CHILD_COUNT: usize> GetConstraintsIter<'a, Constraints>
    for TupleChildren<Constraints, T, CHILD_COUNT>
{
    type ConstraintsIter = std::slice::Iter<'a, Constraints>;
    fn constraints_iter(&'a self) -> Self::ConstraintsIter {
        self.constraints.iter()
    }
}

// This macro is used to generate child impls for a bunch of tuples.
// This lets code like `column((widget_a, widget_b, widget_c))` work.
macro_rules! tuple_impls {
    ( $count: tt, $( ($index: tt, $tuple:ident) ),* ) => {
            impl<
                State,
                Context,
                Constraints: Default + 'static + Copy,
                Drawer,
                $( $tuple: Widget<State, Context, Constraints, Drawer>,)*
            > IntoWidgetChildren<State, Context, Constraints, Drawer> for ($( $tuple,)*)
        {
            type WidgetChildren = TupleChildren<Constraints, ($( $tuple,)*), $count>;
            fn into_widget_children(self) -> Self::WidgetChildren {
                TupleChildren {
                    constraints: [Constraints::default(); $count],
                    children: self,
                }
            }
        }

        impl<
            State,
            Context,
            Constraints: 'static,
            Drawer,
            $( $tuple: Widget<State, Context, Constraints, Drawer>,)*
         > WidgetChildren<State, Context, Constraints, Drawer>
            for TupleChildren<Constraints, ($( $tuple,)*), $count>
        {
            #[allow(unused)]
            fn update(&mut self, state: &mut State, context: &mut Context) {
                $(self.children.$index.update(state, context);)*
            }

            #[allow(unused)]
            fn create_children_and_layout(&mut self, state: &mut State, context: &mut Context) {
                $(self.constraints[$index] = self.children.$index.layout(state, context);)*
            }

            #[allow(unused)]
            fn draw<FUNCTION: FnMut(&Constraints) -> Constraints>(
                &mut self,
                state: &mut State,
                context: &mut Context,
                drawer: &mut Drawer,
                mut f: FUNCTION,
            ) {
                $(self.children.$index.draw(state, context, drawer, f(&self.constraints[$index]));)*
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
