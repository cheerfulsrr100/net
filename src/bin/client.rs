mod util;

use std::net::TcpStream;

use std::io::Result;
use util::str_cli;

fn main() -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    str_cli(&mut stream)?;
    Ok(())
}
