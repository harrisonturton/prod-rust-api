use crate::start_test_server;
use backend::settings::{get_config, get_database_connection_string};
use sqlx::{Connection, PgConnection};

#[tokio::test]
async fn get_user_returns_ok() {
    let addr = start_test_server()
        .await
        .expect("Failed to start test server");
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/user/", addr))
        .send()
        .await
        .expect("Failed to send request");
    assert!(response.status().is_success());

    let config = get_config().expect("Failed to get config");
    let connection_string = get_database_connection_string(&config.database);
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");

    let user: (String,) = sqlx::query_as("SELECT email FROM users")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch first user");
    assert_eq!("harrisonturton@gmail.com", user.0)
}
