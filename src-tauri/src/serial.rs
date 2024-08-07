// src/serial.rs
use serialport::{self, DataBits, FlowControl, Parity, StopBits};
use std::io::{self, Read};
use std::time::Duration;

pub fn read_from_serial_continuous(port_name: &str) -> io::Result<()> {
    let mut port = serialport::new(port_name, 115200)
        .data_bits(DataBits::Eight)
        .flow_control(FlowControl::None)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .timeout(Duration::from_millis(10))
        .open()?;

    let mut serial_buf: Vec<u8> = vec![0; 32];
    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                let data = String::from_utf8_lossy(&serial_buf[..t]);
                println!("{}", data);
            }
            Err(e) => return Err(e),
        }
    }
}
