use flexi_logger::{DeferredNow, Duplicate, FileSpec, Logger, WriteMode};
use log::Record;
// use std::sync::atomic::{Ordering};
use std::{io::Write, process::Command};

// static LOG_FILE_NAME: AtomicString = AtomicString::new();

pub fn init() {
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

    let log_file_spec = FileSpec::default().directory("logs").basename("output").suffix("txt");
    // Save file name to global variable
    // LOG_FILE_NAME.store("test", Ordering::Relaxed);

    Logger::try_with_str("info")
        .unwrap()
        .format(format)
        .log_to_file(log_file_spec)
        .write_mode(WriteMode::BufferAndFlush)
        .duplicate_to_stdout(Duplicate::All)
        .start()
        .unwrap();
}

pub fn open_log_file() {
    Command::new("explorer").arg("logs").spawn().unwrap();
}

pub fn log(message: String, level: &str) {
    match level {
        "debug" => log::debug!("UI: {}", message),
        "info" => log::info!("UI: {}", message),
        "warn" => log::warn!("UI: {}", message),
        "error" => log::error!("UI: {}", message),
        _ => log::info!("UI: {}", message),
    }
}
