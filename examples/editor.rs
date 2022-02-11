/* A work-in-progress example exploring some UI for an editor */

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use koi::*;

#[derive(Component, Clone)]
struct EditorComponent {
    pub selected: Option<Entity>,
    pub expanded: HashSet<Entity>,
}

pub struct NodeState {
    pub has_children: bool,
    pub indentation_level: usize,
}

fn tree<Data, ChildData, Context, Child: Widget<ChildData, Context>, ChildKey: Eq + Hash>(
    build_tree: fn(&mut Data, &mut dyn FnMut(NodeState, &mut ChildData, ChildKey) -> bool),
    create_node: fn() -> Child,
) -> impl Widget<Data, Context> {
    Tree {
        create_node,
        build_tree,
        children: HashMap::new(),
        phantom: std::marker::PhantomData,
    }
}

pub struct Tree<Data, Context, ChildData, Child: Widget<ChildData, Context>, ChildKey: Eq + Hash> {
    create_node: fn() -> Child,
    build_tree: fn(&mut Data, &mut dyn FnMut(NodeState, &mut ChildData, ChildKey) -> bool),
    children: HashMap<ChildKey, (Child, f32, bool)>,
    phantom: std::marker::PhantomData<(Data, Context)>,
}

const INDENTATION_AMOUNT: f32 = 20.;

impl<Data, Context, ChildData, Child: Widget<ChildData, Context>, ChildKey: Eq + Hash>
    Widget<Data, Context> for Tree<Data, Context, ChildData, Child, ChildKey>
{
    fn update(&mut self, data: &mut Data, context: &mut Context) {
        // Because children can only be added and not removed the memory consumed by this will grow as the entities in the scene increases.
        // This is not a problem for now.

        // Update the tree in response to data.
        (self.build_tree)(data, &mut |_, child_data, child_key| {
            let entry = self.children.entry(child_key);
            // The `true` indicates this starts out expanded.
            let v = entry.or_insert_with(|| ((self.create_node)(), 0.0, true));
            v.0.update(child_data, context);
            v.2
        });
    }

    fn layout(
        &mut self,
        data: &mut Data,
        context: &mut Context,
        min_and_max_size: MinAndMaxSize,
    ) -> Vec3 {
        let mut y_height = 0.0;
        let mut width: f32 = 0.0;

        (self.build_tree)(data, &mut |node_state, child_data, child_key| {
            let (child, height, expanded) = self.children.get_mut(&child_key).unwrap();
            let child_size = child.layout(child_data, context, min_and_max_size);
            width =
                width.max(node_state.indentation_level as f32 * INDENTATION_AMOUNT + child_size.x);
            *height = child_size.y;
            y_height += child_size.y;
            *expanded
        });

        Vec3::new(width, y_height, 0.1)
    }
    fn draw(&mut self, data: &mut Data, context: &mut Context, drawer: &mut Drawer, bounds: Box3) {
        let mut y = bounds.min.y;

        (self.build_tree)(data, &mut |node_state, child_data, child_key| {
            let (child, height, expanded) = self.children.get_mut(&child_key).unwrap();
            child.draw(
                child_data,
                context,
                drawer,
                Box3::new(
                    Vec3::new(
                        bounds.min.x + INDENTATION_AMOUNT * node_state.indentation_level as f32,
                        y,
                        0.1,
                    ),
                    Vec3::new(bounds.max.x, y + *height, 0.1),
                ),
            );
            y += *height;
            *expanded
        });
    }
}

fn entity_column<Context: GetStandardStyle + GetFonts + GetStandardInput + Clone>(
) -> impl Widget<World, Context> {
    fit(stack((
        fill(|_| Color::WHITE),
        column(for_each(
            |world: &mut World, per_child| {
                (|mut transforms: Query<Option<&mut Transform>>,
                  editor_component: &mut EditorComponent| {
                    for (e, _) in transforms.entities_and_components_mut() {
                        // Pass the child a flag that can inspected in this scope to see which selection was made.
                        let mut t = (e.clone(), false);
                        per_child(&mut t);
                        if t.1 {
                            editor_component.selected = Some(t.0);
                        }
                    }
                })
                .run(world)
            },
            || {
                button(
                    |child_data: &mut (Entity, bool)| {
                        child_data.1 = true;
                    },
                    |t: &mut (Entity, bool)| format!("Entity {:?}", t.0.index()),
                )
            },
        )),
    )))
}

fn main() {
    App::new().setup_and_run(|world: &mut World| {
        world.spawn(EditorComponent {
            selected: None,
            expanded: HashSet::new(),
        });

        // A camera is needed to display the UI
        world.spawn((Transform::new(), Camera::new(), CameraControls::new()));
        world.spawn((Transform::new(), Camera::new_for_user_interface()));

        let parent = world.spawn((
            Name("Parent"),
            Transform::new().with_position(Vec3::X),
            Mesh::CUBE,
            Material::DEFAULT,
        ));
        let child = world.spawn((
            Name("Child"),
            Transform::new().with_position(Vec3::X),
            Mesh::CUBE,
            Material::DEFAULT,
        ));

        set_parent(world, Some(parent), child);
        use kui::*;

        let mut fonts = Fonts::empty();
        fonts.load_default_fonts();

        let mut style = kui::StandardStyle::default();
        style.primary_text_color = Color::WHITE;
        let mut standard_context =
            kui::StandardContext::new(style, kui::StandardInput::default(), fonts);

        fn recurse_widgets(
            entity: Entity,
            name: &'static str,
            hierarchy_node: HierarchyNode,
            entities: &Query<(
                Option<&mut Transform>,
                Option<&Name>,
                Option<&HierarchyNode>,
            )>,
            call_per_child: &mut dyn FnMut(
                NodeState,
                &mut (Entity, bool, &'static str),
                Entity,
            ) -> bool,
            indentation_level: usize,
        ) {
            let expanded = call_per_child(
                NodeState {
                    has_children: hierarchy_node.last_child().is_some(),
                    indentation_level,
                },
                &mut (entity, false, name),
                entity.clone(),
            );
            if expanded {
                let mut current_child = hierarchy_node.last_child().clone();
                while let Some(child) = current_child {
                    let child_components = entities.get_entity_components(child).unwrap();
                    let child_hierarchy = child_components.2.unwrap();

                    let child_name = child_components.1.map_or("Unnamed", |n| n.0);
                    recurse_widgets(
                        child,
                        child_name,
                        child_hierarchy.clone(),
                        entities,
                        call_per_child,
                        indentation_level + 1,
                    );
                    current_child = child_hierarchy.previous_sibling().clone();
                }
            }
        }
        let mut root_widget = tree(
            |world, call_per_child| {
                (|entities: Query<(
                    Option<&mut Transform>,
                    Option<&Name>,
                    Option<&HierarchyNode>,
                )>,
                  _editor_component: &mut EditorComponent| {
                    for (e, (_, name, hierarchy_node)) in entities.entities_and_components() {
                        let name = name.map_or("Unnamed", |n| n.0);
                        if let Some(hierarchy_node) = hierarchy_node {
                            if hierarchy_node.parent().is_none() {
                                recurse_widgets(
                                    *e,
                                    name,
                                    hierarchy_node.clone(),
                                    &entities,
                                    call_per_child,
                                    0,
                                );
                            }
                        }
                    }
                })
                .run(world)
            },
            || text(|e: &mut (Entity, bool, &'static str)| e.2.to_string()),
        ); //entity_column();

        let mut ui_manager = UIManager::new(world);

        move |event: Event, world| {
            match event {
                Event::FixedUpdate => {
                    ui_manager.update(world, &mut standard_context, &mut root_widget)
                }
                Event::Draw => {
                    ui_manager.layout_and_draw(world, &mut standard_context, &mut root_widget)
                }
                _ => {}
            }
            false
        }
    });
}
