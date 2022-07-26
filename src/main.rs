use crate::unix_rs::{AddressFamily, Protocol, SocketType};

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

        pub fn desc(&self) -> &str {
            match *self {
                Errno::EUNKNOWN => "Unkonw error",
                Errno::EPROTONOSUPPORT => "Protocol not supported.",
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
                Errno::EPROTONOSUPPORT => write!(f, "{} {:?} {}", *self as i32, self, self.desc()),
                Errno::EUNKNOWN => write!(f, "{} {:?} {}", *self as i32, self, self.desc()),
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
    pub enum SocketType {
        Stream = libc::SOCK_STREAM,
        Datagram = libc::SOCK_DGRAM,
    }

    #[repr(i32)]
    pub enum Protocol {
        TCP,
        UDP,
        #[cfg(debug_assertions)]
        Error = 30,
    }

    pub fn socket(family: AddressFamily, ty: SocketType, protocol: Protocol) -> Result<RawFd> {
        Errno::result(unsafe { libc::socket(family as i32, ty as i32, protocol as i32) })
    }

    pub fn connect(fd: RawFd) {}
}

mod green {

    use std::os::unix::prelude::RawFd;

    use crate::unix_rs::{socket, AddressFamily, Protocol, SocketType};

    #[allow(non_snake_case)]
    pub fn Socket(family: AddressFamily, ty: SocketType, protocol: Protocol) -> RawFd {
        match socket(family, ty, protocol) {
            Ok(fd) => fd,
            Err(errno) => panic!("{}", errno),
        }
    }
}

fn main() {
    use crate::green::Socket;

    let fd = Socket(AddressFamily::Inet, SocketType::Stream, Protocol::TCP);
    println!("{}", fd)
}
