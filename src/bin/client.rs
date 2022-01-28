use std::error::Error;
use std::io;
use std::io::{ErrorKind, Write};
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:33369")?;
    // let message = b"Hello from client :D";
    // write(&mut stream, message)?;
    // let bytes = message;
    // println!("{}: {:?}", bytes.len(), bytes);

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let buffer = handle_stdin(input)?;

        write(&mut stream, &buffer)?;
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
