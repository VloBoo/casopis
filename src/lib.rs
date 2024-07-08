use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

pub struct Casopis {
    level: Level,
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
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

impl Casopis {
    pub fn init(level: Level) -> Result<(), SetLoggerError> {
        let mut casopis = Casopis::default();
        casopis.level = level;
        log::set_boxed_logger(Box::new(casopis)).map(|()| log::set_max_level(LevelFilter::Info))
    }
}
