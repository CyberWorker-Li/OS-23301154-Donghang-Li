#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{get_time, yield_};

const SIZE: usize = 50;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("Test malloc app started!");
    let mut arr: [u64; SIZE] = [0; SIZE];
    let start = get_time();
    for i in 0..SIZE {
        arr[i] = (i as u64) * (i as u64);
    }
    let end = get_time();
    println!("Computed squares up to {}", SIZE);
    for i in 0..SIZE {
        if i % 10 == 0 {
            println!("arr[{}] = {}", i, arr[i]);
        }
    }
    println!("Time elapsed: {} ms", end - start);
    println!("Test malloc app finished!");
    0
}
