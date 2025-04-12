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

/// Importing the function to send
/// emails.
use crate::modules::utils::send_email;

/// Importing the function
/// to update a user's
/// profile picture.
use crate::modules::db::users::update_pfp;

/// Importing the function
/// to create a new user.
use crate::modules::db::users::create_user;

/// Importing the function to
/// update the email of a user.
use crate::modules::db::users::update_email;

/// Importing the data structure
/// for returning info on whether
/// a write operation was successful
/// or not.
use crate::modules::responses::StatusResponse;

/// Importing the function to
/// update the username of a user.
use crate::modules::db::users::update_username;

/// Importing the function to
/// update the password of a user.
use crate::modules::db::users::update_password;

/// Importing the structure for modelling instance
/// information in the database for explicit
/// typing.
use crate::modules::models::InstanceInformation;

/// Importing the structure for
/// a payload that is used for
/// changing information about a
/// user.
use crate::modules::payloads::UserChangePayload;

/// Importing the structure for submitting payloads
/// for making auth-related requests.
use crate::modules::payloads::AuthActionPayload;

/// Importing the data structure for
/// submitting payloads for creating 
/// new users.
use crate::modules::payloads::UserCreationPayload;

/// Importing the function to delete
/// a user from the database.
use crate::modules::db::users::delete_user_from_db;

/// Importing the function to
/// retrieve instance information
/// from the database.
use crate::modules::db::general::get_instance_info;

/// Importing the function to update
/// the name of a Cleo user.
use crate::modules::db::users::update_display_name;

/// Importing the data structure for
/// returning information on created
/// users.
use crate::modules::responses::UserCreationResponse;

/// This function is the API service
/// function for creating a user.
/// If the received request and resulting
/// operation are both valid, an instance of
/// the saved extra content field as JSON
/// is returned. In any other case an error 
/// is returned.
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

/// This function is the API service
/// function for editing a user's
/// username. If the received request 
/// and resulting operation are both valid, an 
/// instance of the "StatusResponse" with a 
/// boolean flag is returned as a JSON response. 
/// In any other case an error is returned.
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

/// This function is the API service
/// function for editing a user's
/// display name. If the received request 
/// and resulting operation are both valid, an 
/// instance of the "StatusResponse" with a 
/// boolean flag is returned as a JSON response. 
/// In any other case an error is returned.
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

/// This function is the API service
/// function for editing a user's
/// email address. If the received request 
/// and resulting operation are both valid, an 
/// instance of the "StatusResponse" with a 
/// boolean flag is returned as a JSON response. 
/// In any other case an error is returned.
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

/// This function is the API service
/// function for editing a user's
/// profile picture. If the received request 
/// and resulting operation are both valid, an 
/// instance of the "StatusResponse" with a 
/// boolean flag is returned as a JSON response. 
/// In any other case an error is returned.
#[post("/user/update/picture")]
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

/// This function is the API service
/// function for editing a user's
/// password. If the received request and 
/// resulting operation are both valid, an 
/// instance of the "StatusResponse" with a 
/// boolean flag is returned as a JSON response. 
/// In any other case an error is returned.
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

/// This function is the API service
/// function for deleting a user.
/// If the received request and resulting
/// operation are both valid, an instance of
/// the "StatusResponse" with a boolean flag
/// is returned as a JSON response. In any other
/// case an error is returned.
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
