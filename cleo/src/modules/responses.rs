/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the 
/// "Serialize" trait
/// to derive it.
use serde::Serialize;

/// A data structure
/// to return info
/// in JSON format
/// on a created user.
#[derive(Serialize)]
pub struct UserCreationResponse{
    pub user_id: String,
    pub display_name: String,
    pub is_verified: bool,
    pub username: String,
    pub email_addr: String,
    pub pfp_url: String,
    pub is_admin: bool,
}

/// A data structure
/// to return info
/// in JSON format
/// on a created user
/// for user cration
/// only.
#[derive(Serialize)]
pub struct UserCreationResponseOnly{
    pub user_id: String,
    pub display_name: String,
    pub is_verified: bool,
    pub username: String,
    pub email_addr: String,
    pub pfp_url: String,
    pub is_admin: bool,
    pub key_status_updated: bool
}
/// A data structure
/// to return info
/// on whether a 
/// write operation was
/// successful or not.
#[derive(Serialize)]
pub struct StatusResponse {
    pub is_ok: bool
}

/// A data structure
/// to return info
/// in JSON format
/// on a created API
/// token.
#[derive(Serialize)]
pub struct APITokenResponse {
    pub token_id: String,
    pub token: String,
}

/// A data structure
/// to return info
/// in JSON format
/// on a created user
/// post.
#[derive(Serialize)]
pub struct UserPostResponse{
    pub content_id: String,
    pub content_type: String,
    pub user_id: String,
    pub content_text: String,
}

/// A data structure
/// to return info
/// in JSON format
/// on an additionally-
/// created content field
/// for a user post.
#[derive(Serialize)]
pub struct ExtraContentFieldResponse{
    pub field_id: String,
    pub content_id: String,
    pub field_key: String,
    pub field_value: String
}

/// A data structure
/// to return info
/// in JSON format
/// on a created
/// user file.
#[derive(Serialize)]
pub struct UserFileResponse{
    pub user_id: String,
    pub file_url: String,
    pub file_name: String,
    pub file_id: String
}

/// A data structure
/// to return info
/// in JSON format
/// on a created
/// created user key.
#[derive(Serialize)]
pub struct UserKeyCreationResponse {
    pub key_type: String,
    pub user_key: String
}

/// A data structure
/// to return info
/// in JSON format
/// on all types of
/// users present on
/// an instance.
#[derive(Serialize)]
pub struct InstanceUsersResponse{
    pub users: Vec<UserCreationResponse>
}

/// A data structure
/// to return info
/// in JSON format
/// on all user keys 
/// a user has made.
#[derive(Serialize)]
pub struct UserKeys{
    pub keys: Vec<UserKeyCreationResponse>
}

/// A data structure
/// to return info
/// in JSON format
/// on all user posts 
/// a user has made.
#[derive(Serialize)]
pub struct UserPosts{
    pub posts: Vec<UserPostResponse>
}

/// A data structure
/// to return info
/// in JSON format
/// on all user files
/// a user has uploaded.
#[derive(Serialize)]
pub struct UserFiles{
    pub files: Vec<UserFileResponse>
}

/// A data structure
/// to return info
/// in JSON format
/// on the instance.
#[derive(Serialize)]
pub struct InstanceResponse{
    pub name: String,
    pub hostname: String
}

/// A data structure to
/// return a vector of
/// created user keys
/// as a JSON response.
#[derive(Serialize)]
pub struct UserKeysResponse{
    pub keys: Vec<UserKeyCreationResponse>
}
