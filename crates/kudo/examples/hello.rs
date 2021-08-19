use kudo::*;

struct A;
impl ComponentTrait for A {}

fn main() {
    let mut world = World::new();
    world.spawn((A,));
}
