/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "post"
/// decorator to make a service
/// that accepts "POST" requests.
use actix_web::post;

/// Importing the "Result"
/// enum for Actix Web services.
use actix_web::Result;

/// Importing the "Data"
/// structure to register
/// persistent app data.
use actix_web::web::Data;

/// Importing the "Json"
/// structure to return JSON
/// responses.
use actix_web::web::Json;

/// Importing the function
/// to return a HTTP response.
use actix_web::HttpResponse;

/// Importing this crate's
/// error structure.
use crate::modules::err::CleoErr;

/// Importing the "AppData"
/// structure to register
/// persistent app data.
use crate::modules::units::AppData;

/// Importing the model for
/// users for explicit typing.
use crate::modules::models::CleoUser;

/// Importing the model for
/// user files for explicit typing.
use crate::modules::models::UserFile;

/// Importing the "MultipartForm" trait
/// for explicit typing and to upload files.
use actix_multipart::form::MultipartForm;

/// Importing the structure to upload
/// files via POST request.
use crate::modules::payloads::UserFileUpload;

/// Importing the data structure
/// for returning info on whether
/// a write operation was successful
/// or not.
use crate::modules::responses::StatusResponse;

/// Importing the structure to
/// return info on user files.
use crate::modules::responses::UserFileResponse;

/// Importing the function to
/// delete a user file.
use crate::modules::db::files::delete_user_file;

/// Importing the function to get
/// instance information.
use crate::modules::db::general::get_instance_info;

/// Importing the data structure for submitting
/// a payload for deleting a file a user has
/// uploaded.
use crate::modules::payloads::DeleteUserFilePayload;

/// This function is the API service
/// function for creating a file.
/// If the received request and resulting
/// operation are both valid, an instance of
/// the saved extra content field as JSON
/// is returned. In any other case an error 
/// is returned.
#[post("/files/create")]
pub async fn create_user_file_service(
    MultipartForm(form): MultipartForm<UserFileUpload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    
}

/// This function is the API service
/// function for deleting one of a
/// user's uploaded files. If the received 
/// request and resulting operation are both 
/// valid, an instance of the "StatusResponse" 
/// with a boolean flag is returned as a JSON 
/// response. 
#[post("/files/delete")]
pub async fn delete_user_file_service(
    payload: Json<DeleteUserFilePayload>,
    data: Data<AppData>
) -> HttpResponse {
    let del_op: bool = match delete_user_file(
        &payload.api_token, 
        &payload.file_id,
        &data.pool
    ).await {
        Ok(_op) => true,
        Err(_e) => false
    };
    HttpResponse::Ok().json(StatusResponse{ is_ok: del_op })
}
