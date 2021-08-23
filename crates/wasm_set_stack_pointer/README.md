
## Wasm Set Stack Pointer 
This crate serves a very specific purpose. 

When using WebAssembly in the browser it is possible to share a WebAssembly module between the main 
thread and workers.

Unfortunately the workers and main thread will by-default share a stack. This is no good because they'll 
constantly be writing over each other's stack!

It is possible to allocate a new stack for each worker, but there's no good way to set it...

*until now*

This library links a small snippet of raw WebAssembly that exports a global function ("set_stack_pointer") that can be used to set the stack pointer.

## Usage
Add this crate to your cargo.toml and add `use wasm_set_stack_pointer;` somewhere in your code.

**IMPORTANT**: If you don't add `use wasm_set_stack_pointer;` the linker will strip away the `set_stack_pointer` function.

## Building
This library already has the WebAssembly generated so probably you don't need to do it yourself.

But if you want to build it yourself for some reason you'll need `llvm-mc` installed. `llvm-mc` comes with `llvm` if you have llvm installed.

Then run `llvm-mc -triple=wasm32-unknown-unknown -filetype=obj set_stack_pointer.s > libset_stack_pointer.a`

to place the library in this folder.

## Future

This library may become less relevant when this lands in LLVM 12.0:
[https://github.com/WebAssembly/binaryen/issues/2934](https://github.com/WebAssembly/binaryen/issues/2934)

That flag may be used to force LLVM to export the stack pointer as a WebAssembly global.
