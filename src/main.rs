pub mod dto;
pub mod handlers;

// Import required external crates
extern crate axum;
extern crate diesel;
extern crate dotenv;
extern crate tokio;

use axum::extract::Extension;
use axum::extract::Path;
use axum::http::header::{self, HeaderValue};
use axum::body::Body;

use axum::{
    routing::{get, post, put, patch, delete},
    http::{Response, StatusCode},
    response::IntoResponse,
    Json, Router,
    ServiceExt,
};

use dotenv::dotenv;
use std::env;

// Import Diesel's prelude
use diesel::prelude::*;
use diesel::r2d2::{self, Pool, ConnectionManager};
use diesel::result::Error;
use diesel::pg::PgConnection;

use handlers::ApiHandler;

// Define your API routes
fn routes() {
    Router::new()
        .route("/health", get(ApiHandler::health_check_handler))
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


    // build our application with a route
    let app = routes().with_state(pool);

    // Read the environment variable for port
    let port_str = env::var("REST_PORT").unwrap_or("8080".to_string());

    // Convert the string to u16
    let port: u16 = port_str.parse().expect("Failed to parse port");

    // Bind to the specified IP address and port
    let addr = format!("0.0.0.0:{}", port);
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}
