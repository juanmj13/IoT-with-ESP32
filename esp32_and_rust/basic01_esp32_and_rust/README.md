# Serial Communication with ESP32

This Rust program demonstrates how to communicate with an ESP32 device via a serial port. It sends a message ("Hello World") to the connected device and reads the response.

## Requirements

- Rust 1.56 or higher
- `serialport` crate for serial communication

## Installation

1. Ensure that Rust is installed on your system. You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).
2. Add the `serialport` crate to your `Cargo.toml`:

   ```toml
   [dependencies]
   serialport = "4.0"


## Usage
1. Clone this repository or create a new Rust project and paste the code into the main.rs file.

2. Modify the port_name variable in the code to match your serial port:

    - On Windows, it might be something like "COM3".
    - On Linux, it might be something like "/dev/ttyUSB0"

Compile and run the program

## Author
- Juan Manuel Jimenez (juan.jimenez.c13@gmail.com)
