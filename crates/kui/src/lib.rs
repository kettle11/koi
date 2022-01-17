use kcolor::*;
use kmath::*;

//mod widgets;
//pub use widgets::*;

mod drawer;
pub use drawer::Drawer;

mod style;
pub use style::*;

mod texture_atlas;

mod widgets2;
pub use widgets2::*;

pub trait UIContextTrait: 'static {
    type Data: 'static;
    type Style: GetStandardStyleTrait + 'static;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Font(usize);

pub trait Widget<State, Constraints, Drawer> {
    fn update(&mut self, _state: &mut State) {}
    /// Perform any layout work required and let the parent widget know the Constraints this child requires.
    /// Note that while 'state' is mutable it should not be edited during `layout`.
    fn layout(&mut self, state: &mut State) -> Constraints;

    /// Note that while 'state' is mutable it should not be edited during `draw`.
    fn draw(&mut self, _state: &mut State, _drawer: &mut Drawer, _constraints: Constraints) {}
    fn update_layout_draw(
        &mut self,
        state: &mut State,
        drawer: &mut Drawer,
        initial_constraints: Constraints,
    ) {
        self.update(state);
        self.layout(state);
        self.draw(state, drawer, initial_constraints);
    }
}

pub trait GetStandardDrawer {
    fn standard(&mut self) -> &mut Drawer;
}

impl GetStandardDrawer for Drawer {
    fn standard(&mut self) -> &mut Drawer {
        self
    }
}

pub trait GetStandardConstraints {
    fn standard(&self) -> &StandardConstraints;
    fn standard_mut(&mut self) -> &mut StandardConstraints;
}

impl GetStandardConstraints for StandardConstraints {
    fn standard(&self) -> &StandardConstraints {
        self
    }
    fn standard_mut(&mut self) -> &mut StandardConstraints {
        self
    }
}

#[derive(Clone, Copy)]
pub struct StandardConstraints {
    pub bounds: Box2,
}

impl StandardConstraints {
    pub fn set_size(&mut self, size: Vec2) {
        self.bounds.max = self.bounds.min + size;
    }
}
impl Default for StandardConstraints {
    fn default() -> Self {
        Self { bounds: Box2::ZERO }
    }
}

// Something like this could be defined to pass children into columns, rows, list views, etc?
pub trait WidgetChildren<State, Constraints, Drawer>:
    for<'a> GetConstraintsIter<'a, Constraints>
{
    fn update(&mut self, state: &mut State);
    fn create_children_and_layout(&mut self, state: &mut State);
    fn draw<F: FnMut(&Constraints) -> Constraints>(
        &mut self,
        state: &mut State,
        drawer: &mut Drawer,
        f: F,
    );
}

pub trait GetConstraintsIter<'a, Constraints: 'a> {
    type ConstraintsIter: Iterator<Item = &'a Constraints>;
    fn constraints_iter(&'a self) -> Self::ConstraintsIter;
}

pub fn column<State, Constraints, Drawer, I: IntoWidgetChildren<State, Constraints, Drawer>>(
    children: I,
) -> Consecutive<State, Constraints, Drawer, I::WidgetChildren> {
    Consecutive {
        // This should be reversed for right-to-left.
        direction: Vec2::Y,
        children: children.into_widget_children(),
        phantom: std::marker::PhantomData,
    }
}

pub fn row<State, Constraints, Drawer, I: IntoWidgetChildren<State, Constraints, Drawer>>(
    children: I,
) -> Consecutive<State, Constraints, Drawer, I::WidgetChildren> {
    Consecutive {
        // This should be reversed for right-to-left.
        direction: Vec2::X,
        children: children.into_widget_children(),
        phantom: std::marker::PhantomData,
    }
}

pub struct Consecutive<
    State,
    Constraints,
    Drawer,
    Children: WidgetChildren<State, Constraints, Drawer>,
> {
    direction: Vec2,
    children: Children,
    phantom: std::marker::PhantomData<(State, Constraints, Drawer)>,
}

// This is written a bit more verbosely and less efficiently than necessary to accomodate rows, columns,
// and maybe eventually Z-axis stacks with the same code.
impl<
        State,
        Constraints: GetStandardConstraints + Default,
        Drawer,
        Children: WidgetChildren<State, Constraints, Drawer>,
    > Widget<State, Constraints, Drawer> for Consecutive<State, Constraints, Drawer, Children>
{
    fn update(&mut self, state: &mut State) {
        self.children.update(state)
    }

    fn layout(&mut self, state: &mut State) -> Constraints {
        let mut offset_in_direction = 0.;
        let mut other_dimension_size = Vec2::ZERO;
        self.children.create_children_and_layout(state);
        for child_constraints in self.children.constraints_iter() {
            let child_size = child_constraints.standard().bounds.size();
            let amount_in_directon = child_size.dot(self.direction);
            let non_direction_size = child_size - (amount_in_directon * self.direction);
            other_dimension_size = other_dimension_size.max(non_direction_size);
            offset_in_direction += amount_in_directon;
        }
        let mut constraints = Constraints::default();
        constraints
            .standard_mut()
            .set_size(other_dimension_size + self.direction * offset_in_direction);
        constraints
    }

    fn draw(&mut self, state: &mut State, drawer: &mut Drawer, constraints: Constraints) {
        let mut offset = constraints.standard().bounds.min;
        self.children.draw(state, drawer, |constraints| {
            let mut child_constraints = Constraints::default();
            child_constraints.standard_mut().bounds =
                Box2::new(offset, offset + constraints.standard().bounds.size());
            offset += constraints.standard().bounds.size().dot(self.direction) * self.direction;
            child_constraints
        })
    }
}

pub trait IntoWidgetChildren<State, Constraints, Drawer> {
    type WidgetChildren: WidgetChildren<State, Constraints, Drawer>;
    fn into_widget_children(self) -> Self::WidgetChildren;
}

impl<
        State,
        Constraints: 'static + Default,
        Drawer,
        ChildState,
        Child: Widget<ChildState, Constraints, Drawer>,
    > IntoWidgetChildren<State, Constraints, Drawer>
    for ChildForEach<State, Constraints, Drawer, ChildState, Child>
{
    type WidgetChildren = Self;
    fn into_widget_children(self) -> Self::WidgetChildren {
        self
    }
}

pub struct ChildForEach<
    State,
    Constraints: 'static,
    Drawer,
    ChildState,
    Child: Widget<ChildState, Constraints, Drawer>,
> {
    constraints: Vec<Constraints>,
    children: Vec<Child>,
    call_per_child: fn(&mut State, &mut dyn FnMut(&mut ChildState)),
    create_child: fn() -> Child,
    phantom: std::marker::PhantomData<fn() -> Drawer>,
}

pub fn for_each<
    State,
    Constraints: 'static,
    Drawer,
    ChildState,
    Child: Widget<ChildState, Constraints, Drawer>,
>(
    call_per_child: fn(&mut State, &mut dyn FnMut(&mut ChildState)),
    create_child: fn() -> Child,
) -> ChildForEach<State, Constraints, Drawer, ChildState, Child> {
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
        Constraints: Default,
        Drawer,
        ChildState,
        Child: Widget<ChildState, Constraints, Drawer>,
    > WidgetChildren<State, Constraints, Drawer>
    for ChildForEach<State, Constraints, Drawer, ChildState, Child>
{
    fn update(&mut self, state: &mut State) {
        let mut i = 0;
        (self.call_per_child)(state, &mut |child_state| {
            if let Some(child) = self.children.get_mut(i) {
                child.update(child_state);
            }
            i += 1;
        });
    }

    fn create_children_and_layout(&mut self, state: &mut State) {
        let mut i = 0;
        (self.call_per_child)(state, &mut |child_state| {
            if i >= self.children.len() {
                self.children.push((self.create_child)());
                self.constraints.push(Constraints::default())
            }
            self.constraints[i] = (self.children[i]).layout(child_state);
            i += 1;
        });
    }

    fn draw<F: FnMut(&Constraints) -> Constraints>(
        &mut self,
        state: &mut State,
        drawer: &mut Drawer,
        mut f: F,
    ) {
        let mut i = 0;
        (self.call_per_child)(state, &mut |child_state| {
            // Pass in already calculated child constraints to get final draw constraints.
            let child_constraints = f(&self.constraints[i]);
            self.children[i].draw(child_state, drawer, child_constraints);
            i += 1;
        });
    }
}

impl<
        'a,
        State,
        Constraints: 'a,
        Drawer,
        ChildState,
        Child: Widget<ChildState, Constraints, Drawer>,
    > GetConstraintsIter<'a, Constraints>
    for ChildForEach<State, Constraints, Drawer, ChildState, Child>
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
                Constraints: Default + 'static + Copy,
                Drawer,
                $( $tuple: Widget<State, Constraints, Drawer>,)*
            > IntoWidgetChildren<State, Constraints, Drawer> for ($( $tuple,)*)
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
            Constraints: 'static,
            Drawer,
            $( $tuple: Widget<State, Constraints, Drawer>,)*
         > WidgetChildren<State, Constraints, Drawer>
            for TupleChildren<Constraints, ($( $tuple,)*), $count>
        {
            #[allow(unused)]
            fn update(&mut self, state: &mut State) {
                $(self.children.$index.update(state);)*
            }

            #[allow(unused)]
            fn create_children_and_layout(&mut self, state: &mut State) {
                $(self.constraints[$index] = self.children.$index.layout(state);)*
            }

            #[allow(unused)]
            fn draw<FUNCTION: FnMut(&Constraints) -> Constraints>(
                &mut self,
                state: &mut State,
                drawer: &mut Drawer,
                mut f: FUNCTION,
            ) {
                $(self.children.$index.draw(state, drawer, f(&self.constraints[$index]));)*
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
