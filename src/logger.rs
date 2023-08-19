// filename: logger.rs

use log::{error, info};
use simplelog::{self, Config, LevelFilter};

#[allow(unused)]
pub fn init_logger() {
    simplelog::SimpleLogger::new(LevelFilter::Info, Config::default());
    info!("Logger initialized");
}

#[allow(unused)]
pub fn info(msg: &str) {
    info!("{}", msg);
}

#[allow(unused)]
pub fn error(msg: &str) {
    error!("{}", msg);
}
