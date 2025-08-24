// Checks if the correct annotation for the indirect return ABI is passed to
// llvm. Also checks that the `abi_aarch64_indirect_return`` feature gate allows usage
// of the `aarch64-indirect-return` abi.
//
//@ add-core-stubs
//@ compile-flags: -C no-prepopulate-passes --target aarch64-unknown-none -Copt-level=0
//@ needs-llvm-components: aarch64

#![crate_type = "lib"]
#![no_core]
#![feature(abi_aarch64_indirect_return, no_core, lang_items)]

extern crate minicore;
use minicore::*;

// CHECK: define dso_local void @has_indirect_return_abi(ptr sret %result_location, i64 %value)
#[no_mangle]
pub extern "aarch64-indirect-return" fn has_indirect_return_abi(
    result_location: *mut i64,
    value: i64,
) {
    unsafe { *result_location = value }
}
