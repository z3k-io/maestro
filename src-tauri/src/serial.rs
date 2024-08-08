// src/serial.rs
use serial::prelude::*;
use std::io::{self, BufRead, BufReader};
use std::time::Duration;

pub fn read_continuous<F>(port_name: &str, mut callback: F) -> io::Result<()>
where
    F: FnMut(String) + Send + 'static,
{
    println!("Setting up serial port: {}", port_name);
    let mut port = serial::open(port_name)?;

    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud115200)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;

    port.set_timeout(Duration::from_secs(2))?;

    let mut reader = BufReader::new(port);
    let mut buffer = String::new();

    let mut current_value = String::new();

    loop {
        match reader.read_line(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    let line = buffer.trim().to_string();
                    buffer.clear();
                    if !current_value.eq(&line) {
                        current_value = line.clone();
                        callback(line);
                        continue;
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                // TODO: We need to throw the user a notification, presumably we disconnected.
                println!("Read timed out, continuing...");
            }
            Err(e) => {
                eprintln!("Error reading from serial port: {}", e);
                return Err(e);
            }
        }
    }
}
