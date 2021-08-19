pub use kreflect_common::*;
pub use kreflect_derive::*;

pub trait Reflect {
    fn type_name() -> &'static str;
    fn reflect() -> Value<'static>;
}
