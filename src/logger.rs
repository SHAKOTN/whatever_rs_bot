use log::{Log, Record, Level, Metadata, SetLoggerError, LevelFilter, set_logger, set_max_level};

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
    set_logger(&SimpleLogger).map(|()| set_max_level(LevelFilter::Trace))
}