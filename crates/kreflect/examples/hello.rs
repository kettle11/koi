//use std::any::Any;

use std::collections::HashMap;

use kreflect::*;

trait LoadableAssetTrait {
    type AssetLoader;
}

#[derive(Reflect)]
struct Assets {
    path_to_handle: HashMap<String, bool>,
}

#[derive(Reflect)]
enum TestEnum {
    Thing(f32, f32, f32),
}

#[derive(Reflect)]
struct Thingy {
    f: *mut std::ffi::c_void,
}

fn main() {}
/*
#[derive(Reflect)]
struct Thing2<'a, T> {
    v: &'a T,
}

struct A<T> {
    t: T,
}

#[derive(Reflect)]
struct Thing {
    x: f32,
    y: f32,
}

#[derive(Reflect)]
struct Thing0;

struct Thing1(pub f32, (f32, f32));

#[derive(Reflect)]
enum Test {
    Thing,
    Thing1,
    Thing2 { x: f32, y: f32 },
    Thing4(Vec<f32>),
}

#[derive(Reflect)]
struct Thing3 {
    item: crate::A<f32>,
}

trait SpatialHandle {}
struct Handle<T> {
    t: T,
}

struct Vec3;
struct Sound;

#[derive(Reflect)]
pub struct AudioSource {
    to_play: Vec<(Handle<Sound>, bool)>,
    playing: Vec<Box<dyn SpatialHandle>>,
    last_position: Option<Vec3>,
    volume: f32,
    pub teleported: bool,
}

/*
impl Reflect for Thing {
    fn type_name() -> &'static str {
        todo!()
    }
    fn reflect() -> &'static Value<'static> {
        static DATA: Value<'static> = kreflect::Value::Struct(kreflect::Struct {
            name: "Thing",
            members: Fields::Struct(vec![]),
            generic_lifetimes: vec![],
            generic_types: vec![],
        });
        &DATA
    }
}
*/

/*
impl Thing {
    pub fn name() -> &'static str {
        "Thing"
    }

    pub fn fields_names() -> &'static [&'static str] {
        &["x", "y"]
    }

    pub fn get_field<T: 'static>(&self, name: &str) -> Option<&T> {
        match name {
            "x" => (&self.x as &dyn Any).downcast_ref(),
            "y" => (&self.x as &dyn Any).downcast_ref(),
            _ => None,
        }
    }

    pub fn get_field_mut<T: 'static>(&mut self, name: &str) -> Option<&mut T> {
        match name {
            "x" => (&mut self.x as &mut dyn Any).downcast_mut(),
            "y" => (&mut self.x as &mut dyn Any).downcast_mut(),
            _ => None,
        }
    }
}
*/
/*
#[derive(Reflect)]
#[repr(C)]
enum TestEnum {
    HI,
    Hithere,
}
*/

/*
static THING: Value = Value::Struct(Struct {
    name: "Thing",
    members: Fields::Struct(vec![Field {
        name: "x",
        _type: "f32",
        visibility: Visibility::Private
    }],
    generic_lifetimes: Vec::new(),
    generic_types: Vec::new(),
});
*/
fn main() {
    //  println!("NAME: {}", Thing::type_name());
    //  println!("REFLECTION: {:#?}", Thing::reflect());
    //  let mut x = Thing { x: 10., y: 10. };
    //  // *x.get_field_mut("x").unwrap() = 20. as f32;
}
*/
