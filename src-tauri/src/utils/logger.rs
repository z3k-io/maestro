use flexi_logger::{DeferredNow, Duplicate, FileSpec, Logger, WriteMode};
use log::Record;
use std::{fs, io::Write, path::Path, process::Command};

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
            "{}{} [{}] - {} - {}\x1b[0m\n",
            color_code,
            now.now().format("%Y-%m-%d %H:%M:%S"),
            level,
            record.target(),
            record.args()
        )
    };

    let log_file_spec = FileSpec::default().directory("logs").basename("output").suffix("txt");

    Logger::try_with_str("info")
        .unwrap()
        .format(format)
        .log_to_file(log_file_spec)
        .write_mode(WriteMode::BufferAndFlush)
        .duplicate_to_stdout(Duplicate::All)
        .start()
        .unwrap();

    delete_old_log_files(3);
}

fn delete_old_log_files(keep: usize) {
    let logs_dir = Path::new("logs");
    if !logs_dir.exists() {
        return;
    }

    let mut log_files: Vec<_> = fs::read_dir(logs_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("txt") {
                Some((entry.metadata().unwrap().modified().unwrap(), path))
            } else {
                None
            }
        })
        .collect();

    if log_files.len() <= keep {
        return;
    }

    log_files.sort_by(|a, b| b.0.cmp(&a.0));

    for (_, path) in log_files.into_iter().skip(keep) {
        log::debug!("Deleting old log file: {}", path.display());
        fs::remove_file(path).unwrap();
    }
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
