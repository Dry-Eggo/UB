#![allow(unused)]
mod memory;
mod parser;
mod session;

fn main() {
    let mut allocator = memory::arena::Allocator::new();
    session::session::beginSession(allocator);
}
