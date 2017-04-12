use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

use log;
use log::{ LogRecord, LogLevel, LogMetadata, Log, SetLoggerError, LogLevelFilter };
use chrono::{ Datelike, Timelike, Local };


pub struct Logger {
    file: Mutex<File>
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(| max_log_level | {
        max_log_level.set(LogLevelFilter::Info);
        let logger = match Logger::new() {
            Ok(l) => l,
            Err(_) => panic!("Could not create logger file")
        };
        Box::new(logger)
    })
}

impl Logger {
    pub fn new() -> Result<Logger, ()> {
        let time = Local::now();
        let name = format!("dg_{:04}-{:02}-{:02}_{:02}-{:02}-{:02}.log", time.year(), time.month(), time.day(), time.hour(), time.minute(), time.second());

        let file = match File::create(name) {
            Ok(f) => f,
            Err(_) => return Err(())
        };

        let logger = Logger {
            file: Mutex::new(file)
        };
        Ok(logger)
    }
}

impl Log for Logger {

    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let time = Local::now();
            let msg = format!("[{:04}-{:02}-{:02} {:02}:{:02}:{:02}] {} - {}", time.year(), time.month(), time.day(), time.hour(), time.minute(), time.second(), record.level(), record.args());
            let mut file = self.file.lock().unwrap();
            file.write_fmt(format_args!("{}\n", msg)).unwrap();
        }
    }
}
