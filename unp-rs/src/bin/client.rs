use std::net::TcpStream;

use lib::util::str_cli;
use std::io::Result;

fn main() -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    str_cli(&mut stream)?;
    Ok(())
}
