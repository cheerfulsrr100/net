use crate::errno::{self, errno, IsMinusOne};
use libc::{self, pid_t};

pub fn fork() -> errno::Result<pid_t> {
    unsafe {
        let pid = libc::fork();
        if pid.is_minus_one() {
            Err(errno::Errno::from(errno()))
        } else {
            Ok(pid)
        }
    }
}