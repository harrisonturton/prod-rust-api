use crate::start_test_server;

#[tokio::test]
async fn ping_healthcheck_returns_200() {
    let addr = start_test_server().expect("Failed to spawn app");
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health/", addr))
        .send()
        .await
        .expect("Failed to execute /health request");
    assert!(response.status().is_success());
}

#[tokio::test]
async fn database_healthcheck_returns_500_when_no_database_connected() {
    let addr = start_test_server().expect("Failed to spawn app");
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health/db", addr))
        .send()
        .await
        .expect("Failed to execute /health request");
    assert!(response.status().is_server_error());
}
