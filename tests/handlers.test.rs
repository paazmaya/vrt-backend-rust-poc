use axum::http::StatusCode;
use axum::body::Body;
use axum::test::run;
use serde_json::json;
use crate::{ApiHandler, create_pool, establish_connection};
use crate::dto::*;
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::PgConnection;

use insta::assert_snapshot;

#[test]
fn test_example_snapshot() {
    let data = "Hello, world!";
    assert_snapshot!("example_snapshot", &data);
}


#[tokio::test]
async fn test_health_check_handler() {
    let db_pool = create_pool();
    let api_handler = ApiHandler::new(db_pool);

    let request = axum::test::TestRequest::get().uri("/health").to_request();
    let response = run(&api_handler, request).await;

    assert_eq!(response.status(), StatusCode::OK);
    // Add more assertions based on the expected response
}

// Implement similar unit tests for other handler methods

#[tokio::test]
async fn test_register_user_handler() {
    let db_pool = create_pool();
    let api_handler = ApiHandler::new(db_pool);

    let create_user_dto = CreateUserDto {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        firstName: "Test".to_string(),
        lastName: "User".to_string(),
    };

    let request = axum::test::TestRequest::post()
        .uri("/users/register")
        .set_json(&create_user_dto)
        .to_request();

    let response = run(&api_handler, request).await;

    assert_eq!(response.status(), StatusCode::OK);
    // Add more assertions based on the expected response
}

// Implement similar unit tests for other handler methods

// Define additional unit tests for the remaining handler methods

// Helper function to create a database connection pool for testing
fn create_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = "postgres://your_db_user:your_db_password@localhost/your_db_name";
    let manager = ConnectionManager::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool")
}
