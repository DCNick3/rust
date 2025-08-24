//@ add-core-stubs
//@ needs-llvm-components: aarch64
//@ compile-flags: --target aarch64-unknown-none

#![feature(no_core, abi_aarch64_indirect_return)]
#![no_core]
#![crate_type = "lib"]

extern crate minicore;
use minicore::*;


extern "aarch64-indirect-return" fn non_unit_return(storage_location: *mut i64) -> i64 {
    //~^ ERROR invalid signature for `extern "aarch64-indirect-return"` function
    42
}
