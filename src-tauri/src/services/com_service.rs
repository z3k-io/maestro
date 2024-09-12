use std::{
    collections::HashMap,
    io::{self},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

use crate::{api::events, config};
use tauri::AppHandle;

use super::volume_service;

const DEBOUNCE_INTERVAL: Duration = Duration::from_millis(50);

pub fn listen_serial_input(app_handle: AppHandle) -> () {
    thread::spawn(move || {
        let config = config::get_config();

        let mut current_volumes: HashMap<String, i32> = HashMap::new();

        for session in &config.sessions {
            current_volumes.insert(session.name.clone(), 0);
        }

        let current_volumes = Arc::new(Mutex::new(current_volumes));
        let config = Arc::new(config);
        let app_handle = Arc::new(app_handle);

        let is_first_run = Arc::new(AtomicBool::new(true));

        let on_serial_update_callback = move |data: String| {
            let new_volumes = data.trim().split("|");
            let mut current_volumes = current_volumes.lock().unwrap();

            for (index, new_volume) in new_volumes.enumerate() {
                // look up session name from encoder index
                let session = &config.sessions.iter().find(|s| s.encoder == index as u8).unwrap();
                let current_volume: i32 = *current_volumes.get(&session.name).unwrap();

                let new_volume: i32 = new_volume
                    .trim()
                    .parse::<i32>()
                    .expect(&format!("Failed to parse new volume: <{}>", new_volume));

                if current_volume == new_volume {
                    continue;
                }

                // if volume is negative, session is muted
                if new_volume < 0 {
                    volume_service::set_session_mute(&session.name, true);
                } else {
                    volume_service::set_session_mute(&session.name, false);
                }

                current_volumes.insert(session.name.clone(), new_volume);

                volume_service::set_session_volume(&session.name, new_volume.abs());

                if !is_first_run.load(Ordering::Relaxed) {
                    let audio_sessions = volume_service::get_sessions(&session.name);
                    for audio_session in audio_sessions {
                        events::emit_volume_change_event(&audio_session, (*app_handle).clone());
                    }
                }
            }

            // Set the flag to false after the first run
            is_first_run.store(false, Ordering::Relaxed);
        };

        if let Err(e) = read_continuous(on_serial_update_callback) {
            log::info!("Error reading from serial port: {}", e);
        }
    });
}

fn read_continuous<F>(mut callback: F) -> io::Result<()>
where
    F: FnMut(String) + Send + 'static,
{
    let config = config::get_config();

    let com_port = config.arduino.com_port.clone();
    let baud_rate = config.arduino.baud_rate.clone();

    log::info!("Starting serial read on port: {}", com_port);

    let mut serial_port = serialport::new(com_port, baud_rate).timeout(Duration::from_millis(10)).open()?;

    serial_port.write_data_terminal_ready(true).unwrap();

    let mut buffer = Vec::new();
    let mut last_invoke_time = Instant::now();

    loop {
        let mut serial_buf: Vec<u8> = vec![0; 32];
        match serial_port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                buffer.extend(&serial_buf[..t]);
                while let Some(i) = buffer.iter().position(|&r| r == b'\n') {
                    let line = String::from_utf8_lossy(&buffer[..i]).to_string();
                    buffer.drain(..=i);

                    if last_invoke_time.elapsed() >= DEBOUNCE_INTERVAL {
                        callback(line);
                        last_invoke_time = Instant::now();
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                // No data available, just continue
                continue;
            }
            Err(e) => {
                log::error!("Error reading from serial port: {:?}", e);
                return Err(e);
            }
        }

        // Small sleep to prevent busy-waiting
        thread::sleep(Duration::from_millis(1));
    }
}
