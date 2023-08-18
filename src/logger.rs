// filename: logger.rs

use log::{error, info};
use simplelog::{self, Config, LevelFilter};

#[allow(unused)]
pub fn init_logger() {
    simplelog::SimpleLogger::new(LevelFilter::Info, Config::default());
    info!("Logger initialized");
}

#[allow(unused)]
pub fn log_info(msg: &str) {
    info!("{}", msg);
}

#[allow(unused)]
pub fn log_error(msg: &str) {
    error!("{}", msg);
}
