#[link(name = "set_stack_pointer")]
extern "C" {
    /// Do not call this directly from Rust code.
    /// It's only meant to be used from the Rust host environment.
    #[doc(hidden)]
    pub fn set_stack_pointer(i: i32);
}