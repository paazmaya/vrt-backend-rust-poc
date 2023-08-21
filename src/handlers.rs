use axum::response::IntoResponse;
use axum::{http::StatusCode, Json, Router};

use diesel::pg::PgConnection;
use diesel::r2d2::{Pool, ConnectionManager};

use serde_json::json;
use serde::{Deserialize, Serialize};
use dto::{
    CreateUserDto, UserLoginRequestDto, UserLoginResponseDto, UpdateUserDto, UserDto, AssignRoleDto,
    PaginatedBuildDto, CreateBuildDto, BuildDto, TestRunDto, IgnoreAreaDto, UpdateIgnoreAreasDto,
    UpdateTestRunDto, CreateTestRequestBase64Dto, TestRunResultDto, CreateTestRequestMultipartDto,
    ProjectDto, CreateProjectDto, UpdateProjectDto,
};


#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Serialize, Deserialize)]
struct ApiResponse<T> {
    status: Status,
    data: T,
}

#[derive(Serialize, Deserialize)]
enum Status {
    Ok,
    Error,
}

pub struct ApiHandler {
    db_pool: Pool<ConnectionManager<PgConnection>>,
}

impl ApiHandler {
    pub fn new(db_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        ApiHandler { db_pool }
    }

    pub async fn health_check_handler() -> Json<ApiResponse<String>> {
        Json(ApiResponse {
            status: Status::Ok,
            data: "Health check successful".to_string(),
        })
    }
    /*
    pub async fn register_user_handler() -> Json<ApiResponse<String>> {
        Json(ApiResponse {
            status: Status::Ok,
            data: "User registered successfully".to_string(),
        })
    }
    
    pub async fn login_handler() -> Json<ApiResponse<String>> {
        Json(ApiResponse {
            status: Status::Ok,
            data: "Login successful".to_string(),
        })
    }
    

    pub async fn generate_new_api_key_handler(&self) -> Result<Json<String>, StatusCode> {
        // Implement generating a new API key logic
        // Return the new API key as a string
        Json(ApiResponse {
            status: Status::Ok,
            data: "Login successful".to_string(),
        })
    }

    pub async fn change_password_handler(&self) -> Result<Json<bool>, StatusCode> {
        // Implement changing user password logic
        // Return a boolean indicating success
        Json(ApiResponse {
            status: Status::Ok,
            data: true,
        })
    }

    pub async fn update_user_handler(&self) -> Result<Json<UserLoginResponseDto>, StatusCode> {
        // Implement updating user information logic
        // Return a UserLoginResponseDto
    
        let response_dto = UserLoginResponseDto {
            // Initialize your response fields here
            user_id: "hgfds",
            access_token: "gfdsa",
            token_type: "gfds",
        };

        Ok(Json(response_dto));
    }

    pub async fn delete_user_handler(&self) -> Result<Json<bool>, StatusCode> {
        // Implement deleting a user logic
        // Return a boolean indicating success
        Json(ApiResponse {
            status: Status::Ok,
            data: true,
        })
    }

    pub async fn user_list_handler(&self) -> Result<Json<Vec<UserDto>>, StatusCode> {
        // Implement fetching a list of users logic
        // Return a Vec<UserDto>
        Json(ApiResponse {
            status: Status::Ok,
            data: "Login successful".to_string(),
        })
    }

    pub async fn assign_role_handler(&self) -> Result<Json<UserDto>, StatusCode> {
        // Implement assigning a role to a user logic
        // Return a UserDto with updated role
        Json(ApiResponse {
            status: Status::Ok,
            data: "Login successful".to_string(),
        })
    }

    pub async fn get_test_variations_handler(&self) -> Result<Json<()>, StatusCode> {
        // Implement fetching test variations logic
        // Return appropriate response
        Json(ApiResponse {
            status: Status::Ok,
            data: "Login successful".to_string(),
        })
    }

    pub async fn get_test_variation_details_handler(&self) -> Result<Json<()>, StatusCode> {
        // Implement fetching test variation details logic
        // Return appropriate response
        Json(ApiResponse {
            status: Status::Ok,
            data: "Login successful".to_string(),
        })
    }

    pub async fn merge_test_variations_handler(&self) -> Result<Json<BuildDto>, StatusCode> {
        // Implement merging test variations logic
        // Return a BuildDto representing the merged result
        Json(ApiResponse {
            status: Status::Ok,
            data: "Login successful".to_string(),
        })
    }

    pub async fn delete_test_variation_handler(&self) -> Result<StatusCode, StatusCode> {
        // Implement deleting a test variation logic
        // Return appropriate response
        Json(ApiResponse {
            status: Status::Ok,
            data: "Login successful".to_string(),
        })
    }

    pub async fn get_test_runs_handler(&self) -> Result<Json<()>, StatusCode> {
        // Implement fetching test runs logic
        // Return appropriate response
        Json(ApiResponse {
            status: Status::Ok,
            data: "Login successful".to_string(),
        })
    }

    pub async fn create_test_run_handler(&self) -> Result<Json<TestRunResultDto>, StatusCode> {
        // Implement creating a test run logic
        // Return a TestRunResultDto
        Json(ApiResponse {
            status: Status::Ok,
            data: "Login successful".to_string(),
        })
    }

    pub async fn get_all_projects_handler(&self) -> Result<Json<()>, StatusCode> {
        // Implement fetching all projects logic
        // Return appropriate response
        Json(ApiResponse {
            status: Status::Ok,
            data: "Login successful".to_string(),
        })
    }

    pub async fn create_project_handler(&self) -> Result<Json<()>, StatusCode> {
        // Implement creating a project logic
        // Return appropriate response
        Json(ApiResponse {
            status: Status::Ok,
            data: "Login successful".to_string(),
        })
    }

    pub async fn remove_project_handler(&self) -> Result<Json<()>, StatusCode> {
        // Implement removing a project logic
        // Return appropriate response
        Json(ApiResponse {
            status: Status::Ok,
            data: "Login successful".to_string(),
        })
    }*/

}
