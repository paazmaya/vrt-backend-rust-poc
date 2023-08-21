use axum::prelude::*;
use axum::http::StatusCode;
use axum::extract::Path;
use axum::body::Bytes;
use serde::{Deserialize, Serialize};

use crate::dto::{CreateUserDto, UserLoginRequestDto};
use axum::{extract::Json, Json};

use axum::prelude::*;
use axum::http::StatusCode;
use serde_json::json;

// Import your DTOs and handler methods
mod dto;
mod handlers;
use dto::*;
use handlers::ApiHandler;

use axum::{extract::Json, http::StatusCode, Json};
use serde::Serialize;
use crate::dto::*;


use axum::http::StatusCode;
use serde_json::json;
use crate::dto::*;
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::PgConnection;

use axum::http::StatusCode;
use serde_json::json;
use crate::dto::*;
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::PgConnection;


#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

impl warp::reject::Reject for ErrorResponse {}


pub struct ApiHandler {
    db_pool: Pool<ConnectionManager<PgConnection>>,
}

impl ApiHandler {
    pub fn new(db_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        ApiHandler { db_pool }
    }

    // ... (Other handler methods)

    pub async fn generate_new_api_key_handler(&self) -> Result<Json<String>, StatusCode> {
        // Implement generating a new API key logic
        // Return the new API key as a string
    }

    pub async fn change_password_handler(&self) -> Result<Json<bool>, StatusCode> {
        // Implement changing user password logic
        // Return a boolean indicating success
    }

    pub async fn update_user_handler(&self) -> Result<Json<UserLoginResponseDto>, StatusCode> {
        // Implement updating user information logic
        // Return a UserLoginResponseDto
    }

    pub async fn delete_user_handler(&self) -> Result<Json<bool>, StatusCode> {
        // Implement deleting a user logic
        // Return a boolean indicating success
    }

    pub async fn user_list_handler(&self) -> Result<Json<Vec<UserDto>>, StatusCode> {
        // Implement fetching a list of users logic
        // Return a Vec<UserDto>
    }

    pub async fn assign_role_handler(&self) -> Result<Json<UserDto>, StatusCode> {
        // Implement assigning a role to a user logic
        // Return a UserDto with updated role
    }

    // ... (Other handler methods)

    pub async fn get_test_variations_handler(&self) -> Result<Json<()>, StatusCode> {
        // Implement fetching test variations logic
        // Return appropriate response
    }

    pub async fn get_test_variation_details_handler(&self) -> Result<Json<()>, StatusCode> {
        // Implement fetching test variation details logic
        // Return appropriate response
    }

    pub async fn merge_test_variations_handler(&self) -> Result<Json<BuildDto>, StatusCode> {
        // Implement merging test variations logic
        // Return a BuildDto representing the merged result
    }

    pub async fn delete_test_variation_handler(&self) -> Result<StatusCode, StatusCode> {
        // Implement deleting a test variation logic
        // Return appropriate response
    }

    // ... (Other handler methods)

    pub async fn get_test_runs_handler(&self) -> Result<Json<()>, StatusCode> {
        // Implement fetching test runs logic
        // Return appropriate response
    }

    pub async fn create_test_run_handler(&self) -> Result<Json<TestRunResultDto>, StatusCode> {
        // Implement creating a test run logic
        // Return a TestRunResultDto
    }

    // ... (Other handler methods)

    pub async fn get_all_projects_handler(&self) -> Result<Json<()>, StatusCode> {
        // Implement fetching all projects logic
        // Return appropriate response
    }

    pub async fn create_project_handler(&self) -> Result<Json<()>, StatusCode> {
        // Implement creating a project logic
        // Return appropriate response
    }

    pub async fn remove_project_handler(&self) -> Result<Json<()>, StatusCode> {
        // Implement removing a project logic
        // Return appropriate response
    }

    // ... (Other handler methods)
}
