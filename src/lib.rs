use fmt::{level, module};
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

mod fmt;

pub struct Casopis {
    level: Level,
}

impl Casopis {
    pub fn init(level: Level) -> Result<(), SetLoggerError> {
        let mut casopis = Casopis::default();
        casopis.level = level;
        log::set_boxed_logger(Box::new(casopis)).map(|()| log::set_max_level(LevelFilter::Trace))
    }
}

impl Default for Casopis {
    fn default() -> Self {
        Self { level: Level::Debug }
    }
}

impl Log for Casopis {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let time = chrono::Local::now();
            println!("{:35} {} \x1b[0m[{}]: {}", time.to_rfc3339() , level(record.level()) , module(record.module_path().unwrap()) , record.args());
        }
    }

    fn flush(&self) {}
}
