use std::time::SystemTime;
use log::{Level, Metadata, Record};

pub struct LambdaLogger;

impl log::Log for LambdaLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let now = SystemTime::now();
            let timestamp = now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
            println!("[{}] [{:?}] {} {}", record.level(), timestamp, record.target(), record.args());
        }
    }

    fn flush(&self) {}
}