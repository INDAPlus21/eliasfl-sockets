use std::fs;
use std::io::Read;
use std::path::Path;
use std::{net::TcpListener, thread};

fn main() -> std::io::Result<()> {
    let server = TcpListener::bind("0.0.0.0:33369")?;
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
                    change_led("actpwr").unwrap();
                }
                [2] => {
                    println!("Turning off");
                    change_led("none").unwrap();
                }
                _ => eprintln!("Unknown buffer: {:?}", buf),
            }
        });
    }
    println!("Hello, world! from server");
    Ok(())
}

/// This will echo `power_str` to the file `/sys/class/leds/led0/trigger` if it exists. Equivalent to `echo <power_str> | sudo tee /sys/class/leds/led0/trigger`
///
/// The function will
fn change_led(power_str: &str) -> std::io::Result<()> {
    let path = Path::new("/sys/class/leds/led0/trigger");
    if path.exists() {
        fs::write(path, power_str)?;
    } else {
        eprintln!(
            "Cannot change LED, controll file not found: `/sys/class/leds/led0/trigger`, make sure the server is running on a Raspberry Pi zero"
        );
        eprintln!("Simulating setting power led to: {}", power_str);
    }
    Ok(())
}
