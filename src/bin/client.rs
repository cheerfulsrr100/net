use std::net::TcpStream;

use lib::util::str_cli;

fn main() -> std::io::Result<()> {
    loop {
        let mut stream = TcpStream::connect("127.0.0.1:8080")?;
        println!("connect");
        str_cli(&mut stream)?;
        stream.shutdown(std::net::Shutdown::Both)?;
        println!("shutdown");
    }
}
