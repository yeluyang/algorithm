extern crate env_logger;
#[macro_use]
extern crate log;

use env_logger::Env;
use log::LevelFilter;

fn main() {
    env_logger::init_from_env(Env::default().default_filter_or(LevelFilter::Trace.to_string()));
    info!("starting");
    playground::solution::solution();
}
