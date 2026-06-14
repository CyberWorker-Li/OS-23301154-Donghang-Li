#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::write;

const PAGE_SIZE: usize = 4096;
const BUF_PAGES: usize = 3;
const BUF_LEN: usize = PAGE_SIZE * BUF_PAGES;

#[repr(align(4096))]
struct PageAlignedBuffer([u8; BUF_LEN]);

static mut BUF: PageAlignedBuffer = PageAlignedBuffer([0u8; BUF_LEN]);

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("Test write-cross-page app started!");
    unsafe {
        let buf_ptr = core::ptr::addr_of_mut!(BUF.0) as *mut u8;
        for i in 0..BUF_LEN {
            *buf_ptr.add(i) = b'a' + (i % 26) as u8;
        }

        let start = PAGE_SIZE - 8;
        let len = 32;
        let slice = core::slice::from_raw_parts(buf_ptr.add(start), len);
        let ret = write(1, slice);
        assert_eq!(ret as usize, len);

        println!("");
    }
    println!("Test write-cross-page OK!");
    0
}
