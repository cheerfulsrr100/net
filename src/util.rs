use std::{
    io::{stdin, Read, Result, Write},
    net::TcpStream,
};

pub fn str_echo(stream: &mut TcpStream) -> Result<()> {
    let mut buf = [0; 128];
    match stream.read(&mut buf) {
        Ok(size) => {
            println!(
                "read size: {}, content: {}",
                size,
                String::from_utf8_lossy(&buf)
            );
            stream.write(&buf)?;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn str_cli(stream: &mut TcpStream) -> Result<()> {
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{n} bytes read {input}");
            stream.write(input.as_bytes())?;
            Ok(())
        }
        Err(error) => Err(error),
    }
}
