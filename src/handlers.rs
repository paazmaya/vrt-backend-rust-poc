
use axum::http::StatusCode;
use axum::{extract::State, response::Json};

use rand::Rng;

use diesel::pg::PgConnection;
use diesel::query_dsl::methods::SelectDsl;
use diesel::{SelectableHelper, RunQueryDsl};
use deadpool_diesel::{Pool, Manager};

use serde::{Deserialize, Serialize};

// DTOs
use crate::models::*;

// Table definitions
use crate::schema::users::dsl::*;


#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    status: Status,
    data: T,
}

#[derive(Serialize, Deserialize)]
pub enum Status {
    Ok,
    Error,
}
pub struct ApiHandler;


impl ApiHandler {

/* 
    pub async fn create_user(
        State(pool): State<Pool>,
        Json(new_user): Json<CreateUserDto>,
    ) -> Result<Json<User>, (StatusCode, String)> {
        let conn = pool.get().await.map_err(internal_error)?;
        let res = conn
            .interact(|conn:PgConnection| {
                diesel::insert_into(users::table)
                    .values(new_user)
                    .returning(CreateUserDto::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(internal_error)?
            .map_err(internal_error)?;
        Ok(Json(res))
    }
*/
    /**
     * Return type Result<Json<String>, (StatusCode, String)> 
     * means that the function can return either a JSON string 
     * as a successful result or a tuple containing an HTTP status 
     * code and an error message as an error. 
    */
    pub async fn health_check_handler(
        State(_pool): State<Pool<Manager<PgConnection>>>,
    ) -> Result<Json<HealthResponseDto>, (StatusCode, String)> {
        // Generate a random number between 0 and 1
        let random_number: i32 = rand::thread_rng().gen_range(0..2);

        if random_number == 0 {
            // Simulate an error response
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()))
        } else {
            // Simulate a successful response
            Ok(Json(HealthResponseDto { message: "Health check successful".to_string()}))
        }
    }

    /*
    pub async fn user_list_handler(
        State(pool): State<Pool<Manager<PgConnection>>>,
    ) -> Result<Json<Vec<UserDto>>, (StatusCode, String)> {

        let conn = pool.get().await.map_err(internal_error)?;
        
        let res: Vec<UserDto> = conn
            .interact(|conn: &mut PgConnection| 
                users.select(UserDto::as_select()).load(conn))
            .await
            .map_err(internal_error)?
            .map_err(internal_error)?;
        
        Ok(Json(res))
    }

    pub async fn register_user_handler(
        State(pool): State<Pool<Manager<PgConnection>>>,
        Json(new_user): Json<CreateUserDto>,
    ) -> Result<Json<UserRegisterDto>, (StatusCode, String)> {
        
        let conn = pool.get().await.map_err(internal_error)?;
        let res: UserRegisterDto = conn
            .interact(|conn: &mut PgConnection| {
                diesel::insert_into(users)
                    .values(new_user)
                    .returning(UserRegisterDto::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(internal_error)?
            .map_err(internal_error)?;
        Ok(Json(res))
    }
 */
    /*
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
    */
    /* 
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



/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}