#![no_main]

#[no_mangle]
pub extern "C" fn main(_args: usize, _argv: *const *const u8) -> isize {
  0
}

mod modules;
mod parser;