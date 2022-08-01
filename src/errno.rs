use std::{error::Error, ffi::CStr, fmt::Display, isize};

use libc::c_int;

fn _errno() -> *mut i32 {
    unsafe { libc::__errno_location() }
}

pub fn errno() -> i32 {
    unsafe { *_errno() }
}

pub fn errno_clear() {
    unsafe { *_errno() = 0 }
}

#[derive(Debug)]
pub struct Errno(c_int);

pub trait IsMinusOne {
    fn is_minus_one(&self) -> bool;
}

macro_rules! impl_is_minus_one {
    ($($_type:ty)+) => ($(
        impl IsMinusOne for $_type {
            fn is_minus_one(&self) -> bool {
                *self == -1
            }
        }
    )+)
}

impl_is_minus_one! {i32 isize}

impl Errno {
    pub fn strerror(&self) -> &str {
        unsafe { CStr::from_ptr(libc::strerror(self.0)) }
            .to_str()
            .unwrap()
    }
}

impl Display for Errno {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.strerror())
    }
}

impl From<i32> for Errno {
    fn from(no: i32) -> Self {
        Errno(no)
    }
}

impl Error for Errno {}

pub type Result<T> = std::result::Result<T, Errno>;

#[cfg(test)]
mod tests {
    use super::{Errno, IsMinusOne};

    #[test]
    fn test_strerror_zero() {
        assert_eq!(Errno(0).strerror(), "Success")
    }

    #[test]
    fn test_strerror_minus_one() {
        assert_eq!(Errno(-1).strerror(), "Unknown error -1")
    }

    #[test]
    fn test_is_minus_one() {
        let minus_one = -1i32;
        assert_eq!(minus_one.is_minus_one(), true);
    }
}
