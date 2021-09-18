extern crate log;
extern crate pretty_env_logger;

use death_protocol::start_checkers;
use log::debug;

async fn run() {
    pretty_env_logger::init();
    debug!("start main()");

    start_checkers().await;
}

#[tokio::main]
async fn main() {
    run().await
}
