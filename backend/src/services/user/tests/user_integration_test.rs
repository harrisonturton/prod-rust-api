use crate::base::testlib::start_test_server;
use reqwest::{Client, StatusCode};

#[tokio::test]
async fn get_user_returns_unauthorized_when_not_logged_in() {
    let addr = start_test_server()
        .await
        .expect("Failed to start test server");
    let client = Client::new();
    let response = client
        .get(format!("{}/user/", addr))
        .send()
        .await
        .expect("Failed to send request");
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
