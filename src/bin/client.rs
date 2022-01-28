use std::error::Error;
use std::io;
use std::io::{ErrorKind, Write};
use std::net::TcpStream;
use std::process::exit;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        println!("Usage: {} IP_ADDR", args[0]);
        exit(0);
    }

    let mut stream = TcpStream::connect(format!("{}:33369", args[1]))?;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match handle_stdin(input) {
            Ok(buffer) => {
                write(&mut stream, &buffer)?;
            }
            Err(error) => {
                eprintln!("Not sending request: {}", error.to_string());
            }
        }
    }
}

fn handle_stdin(string: String) -> io::Result<[u8; 1]> {
    match string.trim() {
        "on" => Ok([1]),
        "off" => Ok([2]),
        _ => Err(io::Error::new(ErrorKind::Other, "Unknown sequence")),
    }
}

fn write(mut stream: &TcpStream, message: &[u8]) -> io::Result<()> {
    // First byte is content length
    stream.write(&[message.len() as u8])?;
    stream.write(message)?;
    Ok(())
}
