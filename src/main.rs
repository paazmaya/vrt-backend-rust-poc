
mod handlers;
mod models;
pub mod schema;

use axum::{
    routing::{get, post, put, patch, delete},
    body::Body,
    Router,
};

use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;

use diesel::{pg::PgConnection, Connection};
use deadpool_diesel::{Pool, Manager, Runtime};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


use crate::handlers::ApiHandler;

// this embeds the migrations into the application binary
// the migration path is relative to the `CARGO_MANIFEST_DIR`
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");


// Define your API routes
fn routes() -> Router<Pool<Manager<PgConnection>>, Body> {
    Router::new()
        .route("/health", get(ApiHandler::health_check_handler))
        // .route("/users/register", post(ApiHandler::register_user_handler))
        .route("/users/login", post(ApiHandler::login_handler))
        // .route("/users/newApiKey", get(ApiHandler::generate_new_api_key_handler))
        // .route("/users/password", put(ApiHandler::change_password_handler))
        // .route("/users", put(ApiHandler::update_user_handler))
        // .route("/users", delete(ApiHandler::delete_user_handler))
        // .route("/users/all", get(ApiHandler::user_list_handler))
        // .route("/users/assignRole", patch(ApiHandler::assign_role_handler))
        // .route("/builds", get(ApiHandler::get_builds_handler))
        // .route("/builds", post(ApiHandler::create_build_handler))
        // .route("/builds/{id}", get(ApiHandler::get_build_details_handler))
        // .route("/builds/{id}", delete(ApiHandler::remove_build_handler))
        // .route("/builds/{id}", patch(ApiHandler::update_build_handler))
        // .route("/builds/{id}/approve", patch(ApiHandler::approve_build_handler))
        // .route("/test-variations", get(ApiHandler::get_test_variations_handler))
        // .route("/test-variations/details/{id}", get(ApiHandler::get_test_variation_details_handler))
        // .route("/test-variations/merge", get(ApiHandler::merge_test_variations_handler))
        // .route("/test-variations/{id}", delete(ApiHandler::delete_test_variation_handler))
        // .route("/test-runs", get(ApiHandler::get_test_runs_handler))
        // .route("/test-runs", post(ApiHandler::create_test_run_handler))
        // .route("/test-runs/{id}", get(ApiHandler::get_test_run_details_handler))
        // .route("/test-runs/approve", post(ApiHandler::approve_test_run_handler))
        // .route("/test-runs/reject", post(ApiHandler::reject_test_run_handler))
        // .route("/test-runs/delete", post(ApiHandler::delete_test_run_handler))
        // .route("/test-runs/ignoreAreas/update", post(ApiHandler::update_ignore_areas_handler))
        // .route("/test-runs/ignoreAreas/add", post(ApiHandler::add_ignore_areas_handler))
        // .route("/test-runs/update/{testRunId}", patch(ApiHandler::update_test_run_handler))
        // .route("/test-runs/multipart", post(ApiHandler::create_test_run_multipart_handler))
        // .route("/projects", get(ApiHandler::get_all_projects_handler))
        // .route("/projects", post(ApiHandler::create_project_handler))
        // .route("/projects/{id}", delete(ApiHandler::remove_project_handler))
}



#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "vrt_backend=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables from .env file
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env");
    
    // set up connection pool
    let manager: Manager<PgConnection> = Manager::new(database_url, Runtime::Tokio1);
    let pool: Pool<Manager<PgConnection>> = Pool::builder(manager)
        .build()
        .unwrap();

    // run the migrations on server startup
    {
        let conn = pool.get().await.unwrap();
        conn.interact(|conn: &mut PgConnection| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    // build our application with a route
    let app: Router = routes().with_state(pool);

    // Read the environment variable for port
    let port_str: String = env::var("REST_PORT").unwrap_or("8989".to_string());

    // Convert the string to u16
    let port: u16 = port_str.parse().expect("Failed to parse port");

    // Bind to the specified IP address and port
    let addr: SocketAddr = SocketAddr::from_str(&format!("0.0.0.0:{}", port))
        .expect("Invalid address format");
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}
