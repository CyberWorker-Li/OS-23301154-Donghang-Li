use crate::sbi::console_putchar;
use crate::mm::translated_byte_buffer;
use crate::task::current_user_token;

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    if fd == FD_STDOUT {
        let token = current_user_token();
        let buffers = translated_byte_buffer(token, buf, len);
        for buffer in buffers {
            for &c in buffer.iter() {
                console_putchar(c as usize);
            }
        }
        len as isize
    } else {
        panic!("Unsupported fd in sys_write!");
    }
}
