use crate::sbi::console_putchar;

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    if fd == FD_STDOUT {
        let slice = unsafe { core::slice::from_raw_parts(buf, len) };
        for &c in slice {
            console_putchar(c as usize);
        }
        len as isize
    } else {
        panic!("Unsupported fd in sys_write!");
    }
}
