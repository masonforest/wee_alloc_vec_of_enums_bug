#![no_std]
#![feature(alloc, core_intrinsics, global_allocator, lang_items)]
extern crate rlibc;
#[macro_use] extern crate alloc;
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[lang = "panic_fmt"]
extern "C" fn panic_fmt(_args: ::core::fmt::Arguments, _file: &'static str, _line: u32) -> ! {
    unsafe {
        ::core::intrinsics::abort();
    }
}

pub enum TestEnum {
    U64(u64),
}

#[no_mangle]
pub extern "C" fn run() {
    vec![TestEnum::U64(1)];
}
