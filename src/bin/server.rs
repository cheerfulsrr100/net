use std::net::TcpListener;

use lib::{unistd::fork, util::str_echo};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("listen..");
    loop {
        match listener.accept() {
            Ok((mut stream, _)) => match fork() {
                Ok(pid) => {
                    if pid == 0 {
                        println!("ppid {}, pid {}", unsafe { libc::getppid() }, pid);
                        str_echo(&mut stream)?
                    }
                }
                Err(e) => panic!("{}", e),
            },
            Err(_) => panic!("accept error"),
        }
        println!("end")
    }
}
