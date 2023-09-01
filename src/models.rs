use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Selectable, Insertable};

// Table definitions
use crate::schema::users::dsl::*;
use diesel::SelectableHelper;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Build {
    pub id: String,
    pub ci_build_id: Option<String>,
    pub number: Option<i32>,
    pub branch_name: Option<String>,
    pub status: Option<String>,
    pub project_id: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub user_id: Option<String>,
    pub is_running: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub main_branch_name: String,
    pub builds_counter: i32,
    pub max_build_allowed: i32,
    pub max_branch_lifetime: i32,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub auto_approve_feature: bool,
    pub image_comparison: String,
    pub image_comparison_config: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TestRun {
    pub id: String,
    pub image_name: String,
    pub diff_name: Option<String>,
    pub diff_percent: Option<f64>,
    pub diff_tollerance_percent: f64,
    pub pixel_mismatch_count: Option<i32>,
    pub status: String,
    pub build_id: String,
    pub test_variation_id: Option<String>,
    pub project_id: Option<String>,
    pub merge: bool,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub name: String,
    pub browser: Option<String>,
    pub device: Option<String>,
    pub os: Option<String>,
    pub viewport: Option<String>,
    pub custom_tags: Option<String>,
    pub baseline_name: Option<String>,
    pub comment: Option<String>,
    pub branch_name: String,
    pub baseline_branch_name: Option<String>,
    pub ignore_areas: String,
    pub temp_ignore_areas: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TestVariation {
    pub id: String,
    pub name: String,
    pub branch_name: String,
    pub browser: String,
    pub device: String,
    pub os: String,
    pub viewport: String,
    pub custom_tags: String,
    pub baseline_name: Option<String>,
    pub ignore_areas: String,
    pub project_id: String,
    pub comment: Option<String>,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Baseline {
    pub id: String,
    pub baseline_name: String,
    pub test_variation_id: String,
    pub test_run_id: Option<String>,
    pub user_id: Option<String>,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub api_key: String,
    pub is_active: bool,
    pub role: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

// Enums

#[derive(Debug, Serialize, Deserialize)]
pub enum TestStatus {
    Failed,
    New,
    Ok,
    Unresolved,
    Approved,
    AutoApproved,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ImageComparison {
    PixelMatch,
    LookSame,
    ODiff,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Editor,
    Guest,
}


// Health Response DTO
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct HealthResponseDto {
    pub message: String,
}

// User Register DTO
#[derive(Debug, Serialize, Deserialize, Selectable, Queryable)]
#[diesel(table_name = crate::schema::users)]
pub struct UserRegisterDto {
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

// User Login DTO
#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = crate::schema::users)]
pub struct UserLoginDto {
    pub email: String,
    pub password: String,
}

// User API Key Response DTO
#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = crate::schema::users)]
pub struct UserApiKeyResponseDto {
    pub api_key: String,
}

// User Change Password DTO
#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = crate::schema::users)]
pub struct UserChangePasswordDto {
    pub old_password: String,
    pub new_password: String,
}

// User Update DTO
#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = crate::schema::users)]
pub struct UserUpdateDto {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

// User Role Update DTO
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct UserRoleUpdateDto {
    pub role: String,
}

// User List Response DTO
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct UserListResponseDto {
    pub users: Vec<UserDto>,
}

// User DTO
#[derive(Debug, Serialize, Deserialize, Selectable, Queryable)]
#[diesel(table_name = crate::schema::users)]
pub struct UserDto {
    pub id: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub api_key: String,
    pub is_active: bool,
    pub role: Role,
}


// Build DTO
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct BuildDto {
    pub id: String,
    pub ci_build_id: Option<String>,
    pub number: Option<i32>,
    pub branch_name: Option<String>,
    pub status: Option<String>,
    pub project_id: String,
    pub user_id: Option<String>,
    pub is_running: Option<bool>,
}

// Build Create DTO
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct BuildCreateDto {
    pub ci_build_id: Option<String>,
    pub number: Option<i32>,
    pub branch_name: Option<String>,
    pub status: Option<String>,
    pub project_id: String,
    pub user_id: Option<String>,
    pub is_running: Option<bool>,
}

// Project DTO
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ProjectDto {
    pub id: String,
    pub name: String,
    pub main_branch_name: String,
    pub builds_counter: i32,
    pub max_build_allowed: i32,
    pub max_branch_lifetime: i32,
    pub auto_approve_feature: bool,
    pub image_comparison: String,
    pub image_comparison_config: String,
}

// Test Variation DTO
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TestVariationDto {
    pub id: String,
    pub name: String,
    pub branch_name: String,
    pub browser: String,
    pub device: String,
    pub os: String,
    pub viewport: String,
    pub custom_tags: String,
}

// Test Run DTO
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TestRunDto {
    pub id: String,
    pub image_name: String,
    pub diff_name: Option<String>,
    pub diff_percent: Option<f64>,
    pub diff_tollerance_percent: f64,
    pub pixel_mis_match_count: Option<i32>,
    pub status: String,
    pub build_id: String,
    pub test_variation_id: Option<String>,
    pub project_id: Option<String>,
    pub merge: bool,
    pub name: String,
    pub browser: Option<String>,
    pub device: Option<String>,
    pub os: Option<String>,
    pub viewport: Option<String>,
    pub custom_tags: String,
    pub baseline_name: Option<String>,
    pub comment: Option<String>,
    pub branch_name: String,
    pub baseline_branch_name: Option<String>,
    pub ignore_areas: String,
    pub temp_ignore_areas: String,
}

// Test Variation Details DTO
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TestVariationDetailsDto {
    pub test_variation: TestVariationDto,
    pub baselines: Vec<BaselineDto>,
    pub test_runs: Vec<TestRunDto>,
}

// Test Run Details DTO
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TestRunDetailsDto {
    pub test_run: TestRunDto,
    pub baseline: Option<BaselineDto>,
    pub test_variation: Option<TestVariationDto>,
}

// Baseline DTO
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct BaselineDto {
    pub id: String,
    pub baseline_name: String,
    pub test_variation_id: String,
    pub test_run_id: Option<String>,
    pub user_id: Option<String>,
}

// Create User DTO
#[derive(Debug, Deserialize, Insertable, Selectable, Queryable)]
#[diesel(table_name = crate::schema::users)]
pub struct CreateUserDto {
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

