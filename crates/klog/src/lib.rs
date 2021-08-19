#[cfg(target_arch = "wasm32")]
pub use kwasm::libraries::console::log;

#[macro_export]
macro_rules! log {
    ( $( $arg:tt )* ) => {
        #[cfg(target_arch = "wasm32")]
        klog::log(&format!( $( $arg )* ));
        #[cfg(not(target_arch = "wasm32"))]
        println!("{}", &format!( $( $arg )* ));
    }
}
