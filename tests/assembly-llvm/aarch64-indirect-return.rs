//@ assembly-output: emit-asm

//@ add-core-stubs
//@ compile-flags: --target=aarch64-unknown-linux-gnu
//@ needs-llvm-components: aarch64

#![crate_type = "lib"]
#![no_core]
#![feature(abi_aarch64_indirect_return, no_core, lang_items)]

extern crate minicore;
use minicore::*;

// CHECK-LABEL: has_indirect_return_abi:
#[no_mangle]
pub extern "aarch64-indirect-return" fn has_indirect_return_abi(
    result_location: *mut i64,
    value: i64,
) {
    // CHECK: str x0, [x8]
    // CHECK-NEXT: ret
    unsafe { *result_location = value }
}
