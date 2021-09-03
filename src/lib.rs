use healthchecks::telegram;

mod healthchecks;

pub fn start_checkers() {
    telegram::checkers::send_health_check()
}
