use flexi_logger::{DeferredNow, Duplicate, FileSpec, Logger, WriteMode};
use log::Record;
use std::{io::Write, process::Command};

#[tauri::command]
pub fn log(message: String, level: &str) {
    match level {
        "info" => log::info!("UI: {}", message),
        "error" => log::error!("UI: {}", message),
        _ => log::info!("UI: {}", message),
    }
}

pub fn open_log_file() {
    Command::new("explorer").arg("logs").spawn().unwrap();
}

pub fn init_logger() {
    let format = |write: &mut dyn Write, now: &mut DeferredNow, record: &Record| {
        let level = record.level();
        let color_code = match level {
            log::Level::Error => "\x1b[31m", // Red
            log::Level::Warn => "\x1b[33m",  // Yellow
            log::Level::Info => "\x1b[32m",  // Green
            log::Level::Debug => "\x1b[34m", // Blue
            log::Level::Trace => "\x1b[35m", // Magenta
        };

        write!(
            write,
            "{}{} [{}] - {} - {}\x1b[0m\n", // \x1b[0m resets the color
            color_code,
            now.now().format("%Y-%m-%d %H:%M:%S"),
            level,
            record.target(),
            record.args()
        )
    };

    Logger::try_with_str("info")
        .unwrap()
        .format(format)
        .log_to_file(FileSpec::default().directory("logs").basename("output").suffix("txt"))
        .write_mode(WriteMode::BufferAndFlush)
        .duplicate_to_stdout(Duplicate::All)
        .start()
        .unwrap();
}
