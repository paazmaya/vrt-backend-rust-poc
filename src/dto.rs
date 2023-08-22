use serde::{Deserialize, Serialize};

// Request DTO for user registration
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserDto {
    pub email: String,
    pub password: String,
    pub firstName: String,
    pub lastName: String,
}

// Request DTO for user login
#[derive(Debug, Deserialize, Serialize)]
pub struct UserLoginRequestDto {
    pub email: String,
    pub password: String,
}

// Response DTO for user login and registration
#[derive(Debug, Deserialize, Serialize)]
pub struct UserLoginResponseDto {
    pub id: String,
    pub email: String,
    pub firstName: String,
    pub lastName: String,
    pub apiKey: String,
    pub role: String,
    pub token: String,
}

// Response DTO for build details
#[derive(Debug, Deserialize, Serialize)]
pub struct BuildDto {
    pub id: String,
    pub ciBuildId: String,
    pub number: i32,
    pub branchName: String,
    pub status: String,
    pub projectId: String,
    pub updatedAt: String,
    pub createdAt: String,
    pub userId: String,
    pub passedCount: i32,
    pub unresolvedCount: i32,
    pub failedCount: i32,
    pub isRunning: bool,
    pub merge: bool,
}

// Response DTO for health check
#[derive(Debug, Deserialize, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub info: Option<HealthInfo>,
    pub error: Option<HealthError>,
    pub details: Option<HealthDetails>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthInfo {
    pub database: HealthStatus,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthError {
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthDetails {
    pub database: HealthStatus,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthStatus {
    pub status: String,
}

// Request DTO for updating user information
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserDto {
    pub email: String,
    pub firstName: String,
    pub lastName: String,
}

// Request DTO for assigning a role to a user
#[derive(Debug, Deserialize, Serialize)]
pub struct AssignRoleDto {
    pub id: String,
    pub role: String,
}

// Request DTO for creating a build
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBuildDto {
    pub ciBuildId: String,
    pub branchName: String,
    pub project: String,
}

// Request DTO for creating a test run with base64 image data
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTestRequestBase64Dto {
    pub name: String,
    pub os: String,
    pub browser: String,
    pub viewport: String,
    pub device: String,
    pub branchName: String,
    pub buildId: String,
    pub projectId: String,
    pub diffTollerancePercent: f64,
    pub merge: bool,
    pub ignoreAreas: Vec<IgnoreAreaDto>,
    pub comment: String,
    pub imageBase64: String,
}

// Request DTO for creating a test run with multipart form data
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTestRequestMultipartDto {
    pub name: String,
    pub os: String,
    pub browser: String,
    pub viewport: String,
    pub device: String,
    pub branchName: String,
    pub buildId: String,
    pub projectId: String,
    pub diffTollerancePercent: f64,
    pub merge: bool,
    pub ignoreAreas: Vec<IgnoreAreaDto>,
    pub comment: String,
    pub image: Vec<u8>, // This represents binary image data
}

// DTO for ignore area coordinates
#[derive(Debug, Deserialize, Serialize)]
pub struct IgnoreAreaDto {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

// DTO for pagination information in paginated responses
#[derive(Debug, Deserialize, Serialize)]
pub struct PaginatedDto<T> {
    pub take: i32,
    pub skip: i32,
    pub total: i32,
    pub data: Vec<T>,
}
