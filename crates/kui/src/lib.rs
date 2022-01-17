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

pub fn column<State, Constraints, Drawer, Children: WidgetChildren<State, Constraints, Drawer>>(
    children: Children,
) -> Consecutive<State, Constraints, Drawer, Children> {
    Consecutive {
        direction: Vec2::Y,
        children,
        phantom: std::marker::PhantomData,
    }
}

pub fn row<State, Constraints, Drawer, Children: WidgetChildren<State, Constraints, Drawer>>(
    children: Children,
) -> Consecutive<State, Constraints, Drawer, Children> {
    Consecutive {
        // This should be reversed for right-to-left.
        direction: Vec2::X,
        children,
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
/*
struct TupleChildren<Constraints, A, B> {
    child_a: (A, Constraints),
    child_b: (B, Constraints),
}

struct WidgetForEach<
    State,
    I: Iterator<Item = ChildState>,
    I_MUT: Iterator<Item = ChildState>,
    ChildState,
    Constraints,
    Drawer,
    Child: Widget<ChildState, Constraints, Drawer>,
> {
    get_iterator: fn(&State) -> I,
    get_iterator_mut: fn(&mut State) -> I,
    constraints: Vec<Constraints>,
    children: Vec<Child>,
    phantom: std::marker::PhantomData<fn() -> (ChildState, Constraints, Drawer)>,
}

fn for_each<
    State,
    I: Iterator<Item = ChildState>,
    ChildState,
    Constraints,
    Drawer,
    Child: Widget<ChildState, Constraints, Drawer>,
>(
    get_iterator: fn(&mut State) -> I,
) -> WidgetForEach<State, I, ChildState, Constraints, Drawer, Child> {
    WidgetForEach {
        get_iterator,
        constraints: Vec::new(),
        children: Vec::new(),
        phantom: std::marker::PhantomData,
    }
}

impl<
        State,
        I: Iterator<Item = ChildState>,
        ChildState,
        Constraints: 'static,
        Drawer,
        Child: Widget<ChildState, Constraints, Drawer>,
    > WidgetChildren<State, Constraints>
    for WidgetForEach<State, I, ChildState, Constraints, Drawer, Child>
{
    fn layout(&mut self, state: &State) {
        let iter = (self.get_iterator)(state);
        for data in
        todo!();
    }
    fn draw<F: FnMut() -> Constraints>(&mut self) {
        todo!()
    }
}

impl<
        'a,
        State,
        I: Iterator<Item = ChildState>,
        ChildState,
        Constraints: 'a,
        Drawer,
        Child: Widget<ChildState, Constraints, Drawer>,
    > GetConstraintsIter<'a, Constraints>
    for WidgetForEach<State, I, ChildState, Constraints, Drawer, Child>
{
    type ConstraintsIter = std::slice::Iter<'a, Constraints>;
    fn constraints_iter(&'a self) -> Self::ConstraintsIter {
        self.constraints.iter()
    }
}
*/

// There could be a trait that's effectively "IntoWidgetChildren" that could accept
// different types of items.

// It could be implemented for tuples, and a "foreach" variant that creates children for
// each item in an iterator.
