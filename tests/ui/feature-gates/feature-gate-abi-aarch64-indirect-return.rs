//@ add-core-stubs
//@ needs-llvm-components: aarch64
//@ compile-flags: --target=aarch64-unknown-none --crate-type=rlib
#![no_core]
#![feature(no_core, lang_items)]

extern crate minicore;
use minicore::*;

extern "aarch64-indirect-return" fn fu() {} //~ ERROR extern "aarch64-indirect-return" ABI is experimental

trait T {
    extern "aarch64-indirect-return" fn mu(); //~ ERROR extern "aarch64-indirect-return" ABI is experimental
    extern "aarch64-indirect-return" fn dmu() {} //~ ERROR extern "aarch64-indirect-return" ABI is experimental
}

struct S;
impl T for S {
    extern "aarch64-indirect-return" fn mu() {} //~ ERROR extern "aarch64-indirect-return" ABI is experimental
}

impl S {
    extern "aarch64-indirect-return" fn imu() {} //~ ERROR extern "aarch64-indirect-return" ABI is experimental
}

type TAU = extern "aarch64-indirect-return" fn(); //~ ERROR extern "aarch64-indirect-return" ABI is experimental

extern "aarch64-indirect-return" {} //~ ERROR extern "aarch64-indirect-return" ABI is experimental
