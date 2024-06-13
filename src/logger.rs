extern crate alloc;

use crate::syscalls;
use alloc::format;
use log::{Level, Metadata, Record};
use log::{LevelFilter, SetLoggerError};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let metadata = record.metadata();
        if self.enabled(metadata) {
            let level = metadata.level();
            match level {
                Level::Error => syscalls::debug(format!(
                    "[\x1b[31m{}\x1b[0m {}:{}] {}",
                    record.level(),
                    record.file().unwrap_or("???"),
                    record.line().unwrap_or(0),
                    record.args()
                )),
                Level::Warn => syscalls::debug(format!(
                    "[\x1b[33m{}\x1b[0m  {}:{}] {}",
                    record.level(),
                    record.file().unwrap_or("???"),
                    record.line().unwrap_or(0),
                    record.args()
                )),
                Level::Info => syscalls::debug(format!(
                    "[\x1b[32m{}\x1b[0m  {}:{}] {}",
                    record.level(),
                    record.file().unwrap_or("???"),
                    record.line().unwrap_or(0),
                    record.args()
                )),
                Level::Debug => syscalls::debug(format!(
                    "[\x1b[34m{}\x1b[0m {}:{}] {}",
                    record.level(),
                    record.file().unwrap_or("???"),
                    record.line().unwrap_or(0),
                    record.args()
                )),
                Level::Trace => syscalls::debug(format!(
                    "[\x1b[36m{}\x1b[0m {}:{}] {}",
                    record.level(),
                    record.file().unwrap_or("???"),
                    record.line().unwrap_or(0),
                    record.args()
                )),
            }
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace))
}
