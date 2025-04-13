/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the 
/// "Deserialize" trait
/// to derive it.
use serde::Deserialize;

/// Importing the "Json" structure
/// to accept JSON data with uploaded
/// files.
use actix_multipart::form::json::Json;

/// Importing the "MultipartForm" trait
/// for explicit typing and to upload files.
use actix_multipart::form::MultipartForm;

/// Importing the "TempFile" structure to
/// upload files and save them temporarily.
use actix_multipart::form::tempfile::TempFile;

/// An enum to describe
/// all possible types of
/// content that can be
/// submitted.
#[derive(Deserialize, PartialEq)]
pub enum ContentType{
    Page,
    Post
}

/// A structure for a 
/// payload to
/// create a new user.
#[derive(Deserialize)]
pub struct UserCreationPayload{
    pub username: String,
    pub display_name: String,
    pub password: String,
    pub email_addr: String,
    pub pfp_url: String,
    pub user_key: String,
}

/// A structure for a 
/// payload to
/// change info about a 
/// user.
#[derive(Deserialize)]
pub struct UserChangePayload{
    pub api_token: String,
    pub new_value: String,
}

/// A structure for a 
/// payload to carry
/// out actions requiring
/// higher privileges.
#[derive(Deserialize)]
pub struct AuthActionPayload {
    pub username: String,
    pub password: String    
}

/// A structure for a 
/// payload to delete
/// an API token.
#[derive(Deserialize)]
pub struct DelTokenPayload {
    pub token: String,
    pub username: String,
    pub password: String    
}

/// A structure for submitting
/// a payload for creating a new
/// user post.
#[derive(Deserialize)]
pub struct PostCreationPayload{
    pub api_token: String,
    pub content_type: ContentType,
    pub content_text: String,
}

/// A structure for submitting
/// a payload for editing a
/// user post.
#[derive(Deserialize)]
pub struct UpdatePostPayload{
    pub api_token: String,
    pub content_id: String,
    pub text: String,
}

/// A structure for a 
/// payload to edit
/// a post a user has
/// created. 
#[derive(Deserialize)]
pub struct DeletePostPayload {
    pub api_token: String,
    pub content_id: String,
}

/// A structure for submitting a
/// payload to create extra content
/// fields for a user post.
#[derive(Deserialize)]
pub struct ExtraContentFieldCreationPayload{
    pub api_token: String,
    pub content_id: String,
    pub field_key: String,
    pub field_value: String,
}

/// A structure for submitting a
/// payload to editing an extra content
/// field for a user post.
#[derive(Deserialize)]
pub struct EditExtraContentFieldPayload{
    pub api_token: String,
    pub content_id: String,
    pub field_id: String,
    pub new_value: String,
}

/// A structure for submitting a
/// payload to delete an extra content
/// field for a user post.
#[derive(Deserialize)]
pub struct DeleteExtraContentFieldPayload{
    pub api_token: String,
    pub content_id: String,
    pub field_id: String,
}

/// A structure for submitting a
/// payload to delete a file
/// a user has uploaded.
#[derive(Deserialize)]
pub struct DeleteUserFilePayload{
    pub api_token: String,
    pub file_id: String,
}

/// A structure for submitting a
/// payload to create a new user
/// key.
#[derive(Deserialize)]
pub struct UserKeyPayload{
    pub key_type: String,
    pub api_token: String,
    pub username: String
}

/// A structure for submitting a
/// payload for actions that only
/// require an API token.
#[derive(Deserialize)]
pub struct TokenOnlyPayload {
    pub api_token: String,
}

/// A structure for submitting a
/// payload for deleting a user 
/// key.
#[derive(Deserialize)]
pub struct UserKeyDeletionPayload {
    pub api_token: String,
    pub key_id: String
}

/// A structure for capturing
/// the metadata of a user-uploaded
/// file.
#[derive(Debug, Deserialize)]
pub struct UserFileMetadata{
    pub name: String,
    pub api_token: String
}

/// A structure to simulate a 
/// form for uploading a file.
#[derive(Debug, MultipartForm)]
pub struct UserFileUpload{
    #[multipart(limit = "50MB")]
    pub file: TempFile,
    pub json: Json<UserFileMetadata>
}

