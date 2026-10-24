#![no_std]
#![deny(unsafe_code)]
#![feature(if_let_guard)]

pub mod process;
pub mod syscall;
pub mod ans;

extern crate alloc;

#[ostd::main]
pub fn main() {
    let program_binary = include_bytes!("../target/eval");
    let process = process::Process::new(program_binary);
    process.run();
}