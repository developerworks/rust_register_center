// filename: logger.rs 

use log::{info, error};

pub fn init_logger() {
  simplelog::SimpleLogger::new()
    .with_level(log::LevelFilter::Info)
    .init();  
}

pub fn log_info(msg: &str) {
  info!("{}", msg);
}

pub fn log_error(msg: &str) {
  error!("{}", msg);
}