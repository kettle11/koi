# RUN: /opt/homebrew/opt/llvm/bin/llvm-mc -triple=wasm32-unknown-unknown -filetype=obj set_stack_pointer.s > libset_stack_pointer.a

.globaltype __stack_pointer, i32

set_stack_pointer:
  .globl set_stack_pointer
  .functype set_stack_pointer (i32) -> ()
  local.get 0
  global.set __stack_pointer
  end_function