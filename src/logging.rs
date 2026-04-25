use simplelog::{CombinedLogger, Config as LogConfig, LevelFilter, TermLogger, TerminalMode, WriteLogger, ColorChoice};
use std::fs::File;

pub fn init_logging() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, LogConfig::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Info, LogConfig::default(), File::create("between.log").unwrap()),
    ]).unwrap();
}
