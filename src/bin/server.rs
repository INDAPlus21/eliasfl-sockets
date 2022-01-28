use std::process::Command;
use std::{io::Read, net::TcpListener, thread};

fn main() -> std::io::Result<()> {
    let server = TcpListener::bind("127.0.0.1:33369")?;
    for stream in server.incoming() {
        let mut stream = stream?;
        println!("Connection established!");

        let _handle = thread::spawn(move || loop {
            let mut length = [0; 1];
            stream.read_exact(&mut length).expect("Client disconnected");

            let mut buf = vec![0; length[0].into()];
            stream.read_exact(&mut buf).expect("Invalid content length");
            println!("Request: {}", String::from_utf8_lossy(&buf[..]));

            match buf[..] {
                [1] => {
                    println!("Turning on");
                    turn_on();
                }
                [2] => {
                    println!("Turning off");
                    turn_off();
                }
                _ => panic!("Unknown command: {:?}", buf),
            }
        });
    }
    println!("Hello, world! from server");
    Ok(())
}

fn turn_on() {
    let out = Command::new("cmd")
        .args(["/C", "echo test"])
        .output()
        .unwrap();
    println!("{}", String::from_utf8(out.stdout).unwrap());
}

fn turn_off() {}
