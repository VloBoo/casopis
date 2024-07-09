use log::Level;

pub fn level(level: Level) -> String {
    match level {
        Level::Error => "\x1b[1;97;41mE\x1b[0m",
        Level::Warn => "\x1b[1;97;43mW\x1b[0m",
        Level::Info => "\x1b[1;97;42mI\x1b[0m",
        Level::Debug => "\x1b[1;97;44mD\x1b[0m",
        Level::Trace => "\x1b[1;97;45mT\x1b[0m",
    }
    .to_owned()
}
