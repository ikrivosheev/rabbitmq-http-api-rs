use std::time::Duration;

pub const ENDPOINT: &str = "http://localhost:15672/api";
pub const USERNAME: &str = "guest";
pub const PASSWORD: &str = "guest";

pub fn endpoint() -> String {
    ENDPOINT.to_owned()
}

#[allow(dead_code)]
pub fn await_metric_emission(ms: u64) {
    std::thread::sleep(Duration::from_millis(ms));
}

#[allow(dead_code)]
pub fn await_queue_metric_emission() {
    await_metric_emission(500);
}
