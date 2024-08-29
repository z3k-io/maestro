// src/logger.rs

use flexi_logger::{DeferredNow, Duplicate, FileSpec, Logger, WriteMode};
use log::Record;
use std::{io::Write, process::Command};

pub fn open_log_file() {
    // open::that("logs/output.txt");
    Command::new("explorer").arg("logs").spawn().unwrap();
}

pub fn init_logger() {
    let format = |write: &mut dyn Write, now: &mut DeferredNow, record: &Record| {
        write!(
            write,
            "{} [{}] - {} - {}\n",
            now.now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.target(),
            record.args()
        )
    };

    Logger::try_with_str("info")
        .unwrap()
        .format(format)
        .log_to_file(
            FileSpec::default().directory("logs").basename("output").suffix("txt"), // .suppress_timestamp(),
        )
        .write_mode(WriteMode::BufferAndFlush)
        .duplicate_to_stdout(Duplicate::All)
        .start()
        .unwrap();
}
