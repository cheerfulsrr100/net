use std::{
    io::{stdin, Read, Result, Write},
    mem::size_of,
    net::TcpStream,
};

use libc::c_void;

pub fn str_echo(stream: &mut TcpStream) -> Result<()> {
    let mut buf = [0; 128];
    loop {
        if stream.read(&mut buf)? > 0 {
            print!("{}",String::from_utf8_lossy(&buf));
            stream.write(&buf)?;
        } else {
            return Ok(());
        }
    }
}

pub fn str_cli(stream: &mut TcpStream) -> Result<()> {
    let mut input = String::new();
    let mut out = [0; 128];

    loop {
        if stdin().read_line(&mut input)? > 0 {
            stream.write(input.as_bytes())?;
            stream.read(&mut out)?;
            print!("aaa {}", String::from_utf8_lossy(&out));
            input.clear();
            unsafe {
                libc::memset(out.as_mut_ptr() as *mut c_void, 0, size_of::<[u8; 128]>());
            }
        }
    }
}
