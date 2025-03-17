use std::io::{Read, Write};
use std::time::Duration;
use serialport;

fn main() {
    let port_name = "COM3";  // Change this based on your system (e.g., "/dev/ttyUSB0" for Linux)
    let baud_rate = 115200;

    // Open the serial port
    let mut port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_secs(2))
        .open()
        .expect("Can't open port");

    println!("Sending: Hello World");
    port.write_all(b"Hello World\n").expect("Error writing on port");

    let mut buffer = [0; 100]; // Buffer to receive data
    let bytes_read = port.read(&mut buffer).expect("Error reading from the port");

    if bytes_read > 0 {
        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("ESP32 response: {}", response);
    }
}
