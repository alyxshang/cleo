/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// "File" structure
/// from the standard
/// library to save
/// files to disk.
use std::fs::File;

/// Importing the "get"
/// decorator to make a service
/// that accepts "GET" requests.
use actix_web::get;

/// Importing the "post"
/// decorator to make a service
/// that accepts "POST" requests.
use actix_web::post;

/// Importing the "Result"
/// enum for Actix Web services.
use actix_web::Result;

/// Importing the "actix_files"
/// to serve and open files.
use actix_files as fs;

// Importing the "Data"
/// structure to register
/// persistent app data.
use actix_web::web::Data;

/// Importing the "Json"
/// structure to return JSON
/// responses.
use actix_web::web::Json;

/// Importing the "Path"
/// structure to extract
/// data from URLs.
use actix_web::web::Path;

/// Importing the "NamedFile"
/// structure to serve and open
/// files.
use actix_files::NamedFile;

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

/// Importing the data structure to extract
/// the name and ID of a file from a file
/// request URL.
use crate::modules::extractors::FileExtractor;

/// Importing the function to retrieve a file object
/// from the database given the file's ID.
use crate::modules::db::files::get_file_by_id;

/// Importing the data structure
/// for returning info on whether
/// a write operation was successful
/// or not.
use crate::modules::responses::StatusResponse;

/// Importing the "InstanceInformation"
/// structure to retrieve info about
/// the instance and for explicit 
/// typing.
use crate::modules::models::InstanceInformation;

/// Importing the structure to
/// return info on user files.
use crate::modules::responses::UserFileResponse;

/// Importing the function to
/// delete a user file.
use crate::modules::db::files::delete_user_file;

/// Importing the function to write a new record
/// for a user-uploaded file to the database.
use crate::modules::db::files::create_user_file;

/// Importing the function to get
/// instance information.
use crate::modules::db::general::get_instance_info;

/// Importing the function to retrieve a record
/// for a user given one of their API tokens.
use crate::modules::db::tokens::get_user_from_token;

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
    let info: InstanceInformation = match get_instance_info(&data.pool).await{
        Ok(info) => info,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let file_path: String = format!(
        "{}/{}",
        info.file_dir,
        form.json.name
    );
    let file_url: String = format!(
        "{}/files/serve/{}",
        info.hostname,
        form.json.name
    );
    let _save_op: File = match form.file.file.persist(&file_path){
        Ok(_save_op) => _save_op,
        Err(e) =>return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    }; 
    let created_file: UserFile = match create_user_file(
        &form.json.api_token, 
        &file_path, 
        &file_url,
        &data.pool
    ).await 
    {
        Ok(created_file) => created_file,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let user: CleoUser = match get_user_from_token(
        &form.json.api_token, 
        &data.pool
    ).await 
    {
        Ok(user) => user,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let resp: UserFileResponse = UserFileResponse{
        file_url: file_url,
        file_name: file_path,
        file_id: created_file.file_id,
        user_id: user.user_id
    };
    Ok(HttpResponse::Ok().json(resp))
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

/// This function is the API service
/// function for serving a static file
/// uploaded by a Cleo user onto the server.
/// If the received request and resulting 
/// operation are both valid, an instance 
/// of the "NamedFile" structure is returned
/// as a raw byte array, i.e. as a file response.
/// In any other case, an error is returned.
#[get("/files/serve/{filename}")]
pub async fn static_file_service(
    fname: Path<FileExtractor>,
    data: Data<AppData>
) -> Result<NamedFile, CleoErr>{
    let user_file: UserFile = match get_file_by_id(&fname.filename, &data.pool).await{
        Ok(user_file) => user_file,
        Err(e) => return Err::<NamedFile, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let path = match fs::NamedFile::open(user_file.file_path){
        Ok(path) => path,
        Err(e) => return Err::<NamedFile, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(path)
}
