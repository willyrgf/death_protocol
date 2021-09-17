use death_protocol::start_checkers;
use log::debug;

#[tokio::main]
async fn main() {
    debug!("start main()");

    start_checkers().await;
}
