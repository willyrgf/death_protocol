use healthchecks::telegram;

mod healthchecks;

pub async fn start_checkers() {
    telegram::checkers::send_health_check().await
}
