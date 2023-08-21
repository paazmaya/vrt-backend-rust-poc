use insta::assert_json_snapshot;
use crate::{ApiHandler, create_pool, establish_connection};
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::PgConnection;
use axum::http::StatusCode;
use axum::body::Body;
use axum::test::run;
use serde_json::json;

#[tokio::test]
async fn test_health_check_handler() {
    let db_pool = create_pool();
    let api_handler = ApiHandler::new(db_pool);

    let request = axum::test::TestRequest::get().uri("/health").to_request();
    let response = run(&api_handler, request).await;

    assert_eq!(response.status(), StatusCode::OK);

    // Convert response body to a string
    let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body_str = std::str::from_utf8(&body_bytes).unwrap();

    // Use assert_json_snapshot to perform snapshot testing on the response JSON
    assert_json_snapshot!(body_str, @r###"
        {
            "status": "ok",
            "info": {
                "database": {
                    "status": "up"
                }
            },
            "error": null,
            "details": {
                "database": {
                    "status": "up"
                }
            }
        }
    "###);
}

// Implement similar snapshot tests for other handler methods

// Helper function to create a database connection pool for testing
fn create_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = "postgres://your_db_user:your_db_password@localhost/your_db_name";
    let manager = ConnectionManager::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool")
}
