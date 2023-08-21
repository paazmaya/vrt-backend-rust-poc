use axum::prelude::*;
use axum::http::StatusCode;
use dotenv::dotenv;
use std::env;

// Import Diesel's prelude
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;


// Define your API routes
fn routes() -> Router<Body> {
    route("/health", get(ApiHandler::health_check_handler))
        .route("/users/register", post(ApiHandler::register_user_handler))
        .route("/users/login", post(ApiHandler::login_handler))
        .route("/users/newApiKey", get(ApiHandler::generate_new_api_key_handler))
        .route("/users/password", put(ApiHandler::change_password_handler))
        .route("/users", put(ApiHandler::update_user_handler))
        .route("/users", delete(ApiHandler::delete_user_handler))
        .route("/users/all", get(ApiHandler::user_list_handler))
        .route("/users/assignRole", patch(ApiHandler::assign_role_handler))
        .route("/builds", get(ApiHandler::get_builds_handler))
        .route("/builds", post(ApiHandler::create_build_handler))
        .route("/builds/{id}", get(ApiHandler::get_build_details_handler))
        .route("/builds/{id}", delete(ApiHandler::remove_build_handler))
        .route("/builds/{id}", patch(ApiHandler::update_build_handler))
        .route("/builds/{id}/approve", patch(ApiHandler::approve_build_handler))
        .route("/test-variations", get(ApiHandler::get_test_variations_handler))
        .route("/test-variations/details/{id}", get(ApiHandler::get_test_variation_details_handler))
        .route("/test-variations/merge", get(ApiHandler::merge_test_variations_handler))
        .route("/test-variations/{id}", delete(ApiHandler::delete_test_variation_handler))
        .route("/test-runs", get(ApiHandler::get_test_runs_handler))
        .route("/test-runs", post(ApiHandler::create_test_run_handler))
        .route("/test-runs/{id}", get(ApiHandler::get_test_run_details_handler))
        .route("/test-runs/approve", post(ApiHandler::approve_test_run_handler))
        .route("/test-runs/reject", post(ApiHandler::reject_test_run_handler))
        .route("/test-runs/delete", post(ApiHandler::delete_test_run_handler))
        .route("/test-runs/ignoreAreas/update", post(ApiHandler::update_ignore_areas_handler))
        .route("/test-runs/ignoreAreas/add", post(ApiHandler::add_ignore_areas_handler))
        .route("/test-runs/update/{testRunId}", patch(ApiHandler::update_test_run_handler))
        .route("/test-runs/multipart", post(ApiHandler::create_test_run_multipart_handler))
        .route("/projects", get(ApiHandler::get_all_projects_handler))
        .route("/projects", post(ApiHandler::create_project_handler))
        .route("/projects/{id}", delete(ApiHandler::remove_project_handler))
        // ... Add more routes based on your OpenAPI specification
}


#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize the database connection pool using Diesel and R2D2
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool");

    // Start the Axum server with the defined routes
    let app = routes().layer(axum::AddExtensionLayer::new(pool));
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));

    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server failed");

}
