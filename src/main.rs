use crate::unix_rs::{socket, AddressFamily, SockProtocol, SockType};

pub mod unix_rs {
    use libc;
    use std::{error::Error, fmt::Display, os::unix::prelude::RawFd};

    pub trait ErrnoSentinel: Sized {
        fn sentinel() -> Self;
    }

    impl ErrnoSentinel for i32 {
        fn sentinel() -> Self {
            -1
        }
    }

    #[cfg(target_os = "linux")]
    fn _errno() -> *mut i32 {
        unsafe { libc::__errno_location() }
    }

    pub fn errno() -> i32 {
        unsafe { *_errno() }
    }

    pub fn errno_clear() {
        unsafe { *_errno() = 0 }
    }

    #[repr(i32)]
    #[non_exhaustive]
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Errno {
        EUNKNOWN = 0,
        EPROTONOSUPPORT = libc::EPROTONOSUPPORT,
    }

    impl Errno {
        pub fn result<T: ErrnoSentinel + PartialEq<T>>(v: T) -> Result<T> {
            if v == T::sentinel() {
                Err(Errno::from(errno()))
            } else {
                Ok(v)
            }
        }
    }

    impl From<i32> for Errno {
        fn from(f: i32) -> Self {
            match f {
                libc::EPROTONOSUPPORT => Errno::EPROTONOSUPPORT,
                _ => Errno::EUNKNOWN,
            }
        }
    }

    impl Error for Errno {}

    impl Display for Errno {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match *self {
                Errno::EPROTONOSUPPORT => {
                    write!(
                        f,
                        "{} {:?} {}",
                        *self as i32, self, "Protocol not supported."
                    )
                }
                Errno::EUNKNOWN => write!(f, "{} {:?} {}", *self as i32, self, "Unkonw error"),
            }
        }
    }

    pub type Result<T> = std::result::Result<T, Errno>;

    #[repr(i32)]
    #[non_exhaustive]
    pub enum AddressFamily {
        Inet = libc::AF_INET,
    }

    #[repr(i32)]
    #[non_exhaustive]
    pub enum SockType {
        Stream = libc::SOCK_STREAM,
        Datagram = libc::SOCK_DGRAM,
    }

    #[repr(i32)]
    pub enum SockProtocol {
        TCP,
        UDP,
    }

    pub fn socket(family: AddressFamily, ty: SockType, protocol: SockProtocol) -> Result<RawFd> {
        Errno::result(unsafe { libc::socket(family as i32, ty as i32, protocol as i32) })
    }

    pub fn connect(fd: RawFd) {
        
    }
}

fn main() {
    let fd = socket(AddressFamily::Inet, SockType::Stream, SockProtocol::TCP);
    println!("{}", fd.unwrap());
}
