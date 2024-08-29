// src/serial.rs

use std::borrow::Borrow;
use std::io::{self, BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time::{Duration, Instant};

use crate::config;

const DEBOUNCE_INTERVAL: Duration = Duration::from_millis(50);

pub fn read_continuous<F>(mut callback: F) -> io::Result<()>
where
    F: FnMut(String) + Send + 'static,
{
    let config = config::get_config();

    let com_port = config.com_port.clone();
    let baud_rate = config.baud_rate.clone();

    log::info!("Starting serial read on port: {}", com_port);

    let mut serial_port = serialport::new(com_port, baud_rate)
        .timeout(Duration::from_millis(100))
        .open()?;

    serial_port.write_data_terminal_ready(true);

    let reader = BufReader::new(serial_port);

    // Shared state for debouncing
    let latest_data = Arc::new(Mutex::new(None));
    let last_invoke_time = Arc::new(Mutex::new(Instant::now()));

    let latest_data_clone = Arc::clone(&latest_data);
    let last_invoke_time_clone = Arc::clone(&last_invoke_time);

    thread::spawn(move || {
        loop {
            let mut data = latest_data_clone.lock().unwrap();
            let mut last_time = last_invoke_time_clone.lock().unwrap();

            if let Some(line) = data.take() {
                // Only process if sufficient tim has passed for debounce
                if last_time.elapsed() >= DEBOUNCE_INTERVAL {
                    callback(line);
                    *last_time = Instant::now();
                }
            }
        }
    });

    // Read from the serial port
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut data = latest_data.lock().unwrap();
                *data = Some(line);
            }
            Err(e) => {
                log::info!("Error reading from serial port: {:?}", e);
            }
        }
    }

    Ok(())
}
