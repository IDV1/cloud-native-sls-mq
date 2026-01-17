mod config_process;
mod error;
mod log;
mod command;
mod tools;

use crate::config_process::config::config_handle;
use clap::Parser;

fn main() {
    // 启动
    let _run_server  = command::server::server::Args::parse();
    let _config = config_handle::try_init_path(&_run_server.config);
    log::log::logging::init_logger("log_file/log4rs.yaml");
}
