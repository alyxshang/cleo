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

/// Importing the "Path"
/// structure to read URL
/// paths.
use actix_web::web::Path;

/// Importing the "Json"
/// structure to return JSON
/// responses.
use actix_web::web::Json;

/// Importing this crate's
/// error structure.
use crate::err::CleoErr;

/// Importing the "AppData"
/// structure to register
/// persistent app data.
use crate::units::AppData;

/// Importing the function
/// to update a user's
/// profile picture.
use crate::rw::update_pfp;

/// Importing the function
/// to create a new user.
use crate::rw::create_user;

/// Importing the model for
/// user keys for explicit
/// typing.
use crate::models::UserKey;

/// Importing the model for
/// users for explicit typing.
use super::models::CleoUser;

/// Importing the model for
/// user files for explicit typing.
use super::models::UserFile;

/// Importing the model for
/// user posts for explicit
/// typing.
use super::models::UserPost;

/// Importing the function
/// to return a HTTP response.
use actix_web::HttpResponse;

/// Importing the function to
/// update the email of a user.
use super::rw::update_email;

/// Importing the function to
/// delete an API token.
use super::rw::delete_token;

/// Importing the function
/// to retrieve all the keys
/// a user has made.
use super::rw::get_user_keys;

/// Importing the function to
/// send emails.
use super::utils::send_email;

/// Importing the function
/// to edit the password of
/// the user for the instance's
/// SMTP server.
use super::rw::edit_smtp_pass;

/// Importing the function to
/// retrieve all the posts a 
/// user has made.
use super::rw::get_user_posts;

/// Importing the function to
/// retrieve all the files 
/// a user has uploaded.
use super::rw::get_user_files;

/// Importing the data structure
/// for returning data on all keys
/// a user has created.
use super::responses::UserKeys;

/// Importing the function to
/// update the username of a user.
use super::rw::update_username;

/// Importing the function to
/// create a user key.
use super::rw::create_user_key;

/// Importing the function to
/// delete a user key.
use super::rw::delete_user_key;

/// Importing the function to
/// update the password of a user.
use super::rw::update_password;

/// Importing the data structure
/// for returning data on all posts
/// a user has created.
use super::responses::UserPosts;

/// Importing the function to
/// delete a user file.
use super::rw::delete_user_file;

/// Importing the enum to describe
/// all possible types of content.
use super::payloads::ContentType;

/// Importing the model for
/// user API keys for explicit
/// typing.
use super::models::UserAPIToken;

/// Importing the function to
/// create a post for a user.
use super::rw::create_user_post;

/// Importing the function to
/// update the text of post.
use super::rw::update_post_text;

/// Importing the data structure
/// for returning data on all files
/// a user has created.
use super::responses::UserFiles;

/// Importing the function to get
/// instance information.
use super::rw::get_instance_info;

/// Importing the function to get
/// a list of instance users.
use super::rw::get_instance_users;

/// Importing the function to edit
/// the name of an instance.
use super::rw::edit_instance_name;

/// Importing the function to update
/// username of the instance's SMTP
/// server.
use super::rw::edit_smtp_username;

/// Importing the function to delete
/// a user from the database.
use super::rw::delete_user_from_db;

/// Importing the function to update
/// the name of a Cleo user.
use super::rw::update_display_name;

/// Importing the function to retrieve
/// a list of instance admins.
use super::rw::get_instance_admins;

/// Importing the structure for
/// submitting payloads to
/// carry out actions for
/// user keys.
use super::payloads::UserKeyPayload;

/// Importing the data structure
/// for returning info on whether
/// a write operation was successful
/// or not.
use super::responses::StatusResponse;

/// Importing the data structure for
/// submitting a payload that contains
/// info for deleting an API token.
use super::payloads::DelTokenPayload;

/// Importing the structure for 
/// submitting payloads for actions
/// that only require an API token.
use super::payloads::TokenOnlyPayload;

/// Importing the function to edit
/// the hostname of an instance.
use super::rw::edit_instance_hostname;

/// Importing the data structure for
/// submitting a payload for updating
/// a post a user has made.
use super::payloads::UpdatePostPayload;

/// Importing the data structure for
/// submitting a payload for deleting
/// a post a user has made.
use super::payloads::DeletePostPayload;

/// Importing the data structure for
/// returning information a created 
/// API token.
use super::responses::APITokenResponse;

/// Importing the structure to return
/// info on an instance.
use super::responses::InstanceResponse;

/// Importing the data structure for
/// returning information on created
/// posts.
use super::responses::UserPostResponse;

/// Importing the structure to
/// return info on user files.
use super::responses::UserFileResponse;

/// Importing the structure for a 
/// payload to carry out actions
/// requiring higher privileges.
use super::payloads::AuthActionPayload;

/// Importing the model containing info
/// on the instance.
use super::models::InstanceInformation;

/// Importing the structure for
/// a payload that is used for
/// changing information about a
/// user.
use super::payloads::UserChangePayload;

/// Importing the function to create
/// an API token for a Cleo user.
use super::rw::create_api_token_for_user;

/// Importing the data structure for
/// submitting payloads for creating 
/// new users.
use crate::modules::payloads::UserCreationPayload;

/// Importing the data structure for submitting
/// a payload for deleting a file a user has
/// uploaded.
use crate::modules::payloads::DeleteUserFilePayload;

/// Importing the data structure for
/// returning information on created
/// users.
use crate::modules::responses::UserCreationResponse;

/// This function contains the
/// service function for creating
/// a new user. If this operation
/// fails, an HTTP error is returned.
#[post("/user/create")]
pub async fn create_user_service(
    payload: Json<UserCreationPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let user: CleoUser = match create_user(
        &payload.username, 
        &payload.display_name, 
        &payload.password, 
        &payload.email_addr, 
        &payload.pfp_url, 
        &payload.user_key, 
        &data.pool
    ).await {
        Ok(user) => user,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let instance_info: InstanceInformation = match get_instance_info(&data.pool).await {
        Ok(instance_info) => instance_info,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let body: String = "".to_string();
    let send_mail: bool = match send_email(
        &instance_info.smtp_username, 
        &instance_info.smtp_pass, 
        &format!("Account verification for {}", &instance_info.instance_name), 
        &body, 
        &user.email_addr, 
        &instance_info.smtp_server
    ).await {
        Ok(send_mail) => send_mail,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    if send_mail{
        let user_obj: UserCreationResponse = UserCreationResponse{
            user_id: user.user_id,
            display_name: user.display_name,
            is_verified: user.is_verified,
            username: user.username,
            email_addr: user.email_addr,
            pfp_url: user.pfp_url,
            is_admin: user.is_admin
        };
        Ok(HttpResponse::Ok().json(user_obj))
    }
    else {
        return Err::<HttpResponse, CleoErr>(CleoErr::new(&"Account creation failure.".to_string()))
    }
    
}

/// This function contains the service
/// function that attempts to edit a 
/// user's username. If this operation
/// fails, an HTTP error is returned.
#[post("/user/update/username")]
pub async fn update_username_service(
    payload: Json<UserChangePayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut result: bool = false;
    let _update_op: () = match update_username(
        &payload.api_token, 
        &payload.new_value, 
        &data.pool
    ).await {
        Ok(_update_op) => {result = true},
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))
}

/// This function contains the 
/// service function that attempts
/// to edit a user's display name.
/// If this operation fails, an 
/// HTTP error is returned.
#[post("/user/update/name")]
pub async fn update_name_service(
    payload: Json<UserChangePayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut result: bool = false;
    let _update_op: () = match update_display_name(
        &payload.api_token, 
        &payload.new_value, 
        &data.pool
    ).await {
        Ok(_update_op) => {result = true},
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))
}

/// This function contains the 
/// service function that attempts
/// to edit a user's email address.
/// If this operation fails, an 
/// HTTP error is returned.
#[post("/user/update/email")]
pub async fn update_email_service(
    payload: Json<UserChangePayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut result: bool = false;
    let _update_op: () = match update_email(
        &payload.api_token, 
        &payload.new_value, 
        &data.pool
    ).await {
        Ok(_update_op) => {result = true},
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))
}

/// This function contains the 
/// service function that attempts
/// to edit a user's profile picture.
/// If this operation fails, an 
/// HTTP error is returned.
#[post("/user/update/pfp")]
pub async fn update_pfp_service(
    payload: Json<UserChangePayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut result: bool = false;
    let _update_op: () = match update_pfp(
        &payload.api_token, 
        &payload.new_value, 
        &data.pool
    ).await {
        Ok(_update_op) => {result = true},
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))
}

/// This function contains the 
/// service function that attempts
/// to edit a user's password.
/// If this operation fails, an 
/// HTTP error is returned.
#[post("/user/update/password")]
pub async fn update_password_service(
    payload: Json<UserChangePayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut result: bool = false;
    let _update_op: () = match update_password(
        &payload.api_token, 
        &payload.new_value, 
        &data.pool
    ).await {
        Ok(_update_op) => {result = true},
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))
}

/// This function contains
/// the service function that
/// attempts to delete a user.
/// If this operation fails, 
/// an HTTP error is returned.
#[post("/user/delete")]
pub async fn delete_user_service(
    payload: Json<AuthActionPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut result: bool = false;
    let _del_op: () = match delete_user_from_db(
        &payload.username, 
        &payload.password, 
        &data.pool
    ).await {
        Ok(_del_op) => {result = true},
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))
}

/// This function contains the
/// service function that attempts
/// to create an API token for a user.
/// If this operation fails, an error
/// is returned.
#[post("/api_token/create")]
pub async fn create_api_token_service(
    payload: Json<AuthActionPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let token: UserAPIToken = match create_api_token_for_user(
        &payload.username, 
        &payload.password, 
        &data.pool
    ).await {
        Ok(token) => token,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let resp: APITokenResponse = APITokenResponse{
        token_id: token.token_id,
        token: token.token
    };
    Ok(HttpResponse::Ok().json(resp))
}

/// This function contains the 
/// service function that attempts
/// to delete an API token for a user.
/// If this operation fails, an HTTP
/// error is returned.
#[post("/api_token/delete")]
pub async fn delete_api_token_service(
    payload: Json<DelTokenPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut result: bool = false;
    let _del_op: () = match delete_token(
        &payload.token, 
        &payload.username,
        &payload.password, 
        &data.pool
    ).await {
        Ok(_del_op) => {result = true},
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))
}

/// This function contains the 
/// service function that attempts
/// to delete a file a user has
/// uploaded. If this operation
/// fails, an error is returned.
#[post("/user/files/delete")]
pub async fn delete_user_file_service(
    payload: Json<DeleteUserFilePayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut result: bool = false;
    let _del_op: () = match delete_user_file(
        &payload.api_token, 
        &payload.file_id,
        &data.pool
    ).await {
        Ok(_del_op) => {result = true},
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))
}

/// This function contains the
/// service functions that fetches
/// information on all files a user
/// has uploaded. If this operation fails,
/// an error is returned.
#[get("/info/files")]
pub async fn get_user_files_service(
    payload: Json<TokenOnlyPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut sanitized: Vec<UserFileResponse> = Vec::new();
    let files: Vec<UserFile> = match get_user_files(&payload.api_token, &data.pool).await {
        Ok(files) => files,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let info: InstanceInformation = match get_instance_info(&data.pool).await {
        Ok(info) => info,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    for file in files {
        let resp_file: UserFileResponse = UserFileResponse{
           user_id: file.user_id,
           file_url: format!("{}{}", &info.hostname, &file.file_path),
           file_name: file.file_path
        };
        sanitized.push(resp_file);
    }
    let resp: UserFiles = UserFiles{ files: sanitized };
    Ok(HttpResponse::Ok().json(resp))
}
