#![no_std]

extern crate pwasm_ethereum as ext;

#[no_mangle]
pub unsafe extern "C" fn call() {
    ext::ret(b"Hello world");
}

#[no_mangle]
pub unsafe extern "C" fn deploy() {
}