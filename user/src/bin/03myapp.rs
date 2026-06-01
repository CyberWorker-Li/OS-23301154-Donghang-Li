#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("This is my custom application! -- 23301154 Donghang Li");
    for i in 0..5 {
        println!("Loop iteration: {}", i);
    }
    0
}
