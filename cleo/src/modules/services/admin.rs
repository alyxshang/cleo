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

/// Importing the data structure
/// for returning info on whether
/// a write operation was successful
/// or not.
use crate::modules::responses::StatusResponse;

/// Importing the structure for 
/// submitting payloads for actions
/// that only require an API token.
use crate::modules::payloads::TokenOnlyPayload;

/// Importing the function
/// to edit the password of
/// the user for the instance's
/// SMTP server.
use crate::modules::db::admin::edit_smtp_pass;

/// Importing the structure for
/// a payload that is used for
/// changing information about a
/// user.
use crate::modules::payloads::UserChangePayload;

/// Importing the function to get
/// a list of instance users.
use crate::modules::db::admin::get_instance_users;

/// Importing the function to edit
/// the name of an instance.
use crate::modules::db::admin::edit_instance_name;

/// Importing the function to update
/// username of the instance's SMTP
/// server.
use crate::modules::db::admin::edit_smtp_username;

/// Importing the function to retrieve
/// a list of instance admins.
use crate::modules::db::admin::get_instance_admins;

/// Importing the data structure for
/// returning information on created
/// users.
use crate::modules::responses::UserCreationResponse;

/// Importing the structure to return info on
/// all types of users present on an instance.
use crate::modules::responses::InstanceUsersResponse;

/// Importing the function to edit
/// the hostname of an instance.
use crate::modules::db::admin::edit_instance_hostname;

/// Importing the function to update
/// the address of the instance's SMTP
/// server.
use crate::modules::db::admin::edit_instance_smtp_server;
 
/// This function is the API service
/// function for retrieving a list of
/// all admin users on the current Cleo instance.
/// If the received request and resulting
/// operation are both valid, a vector with
/// instances of the "UserCreationResponse" structure
/// as a JSON response is returned. In any other
/// case an error is returned.
#[post("/instance/admins")]
pub async fn get_instance_admins_service(
    payload: Json<TokenOnlyPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr>{
    let admins: Vec<CleoUser> = match get_instance_admins(&payload.api_token, &data.pool).await {
        Ok(admins) => admins,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let mut result: Vec<UserCreationResponse> = Vec::new();
    for admin in admins {
        let user: UserCreationResponse = UserCreationResponse{
            user_id: admin.user_id,
            display_name: admin.display_name,
            is_verified: admin.is_verified,
            username: admin.username,
            email_addr: admin.email_addr,
            pfp_url: admin.pfp_url,
            is_admin: admin.is_admin
        };
        result.push(user);
    }
    Ok(HttpResponse::Ok().json(InstanceUsersResponse{ users: result }))    
}

/// This function is the API service
/// function for retrieving a list of
/// all users on the current Cleo instance.
/// If the received request and resulting
/// operation are both valid, a vector with
/// instances of the "UserCreationResponse" structure
/// as a JSON response is returned. In any other
/// case an error is returned.
#[post("/instance/users")]
pub async fn get_instance_users_service(
    payload: Json<TokenOnlyPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr>{
    let admins: Vec<CleoUser> = match get_instance_users(&payload.api_token, &data.pool).await {
        Ok(admins) => admins,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let mut result: Vec<UserCreationResponse> = Vec::new();
    for admin in admins {
        let user: UserCreationResponse = UserCreationResponse{
            user_id: admin.user_id,
            display_name: admin.display_name,
            is_verified: admin.is_verified,
            username: admin.username,
            email_addr: admin.email_addr,
            pfp_url: admin.pfp_url,
            is_admin: admin.is_admin
        };
        result.push(user);
    }
    Ok(HttpResponse::Ok().json(InstanceUsersResponse{ users: result }))    
}

/// This function is the API service
/// function for editing the name in use
/// on the current Cleo instance. If 
/// the received request and resulting
/// operation are both valid, an instance
/// of the "StatusResponse" as a JSON
/// response is returned. 
#[post("/instance/edit/name")]
pub async fn edit_instance_name_service(
    payload: Json<UserChangePayload>,
    data: Data<AppData>
) -> HttpResponse {
    let update_op: bool = match edit_instance_name(
        &payload.api_token, 
        &payload.new_value,
        &data.pool
    ).await {
        Ok(_op) => true,
        Err(_e) => false
    };
    HttpResponse::Ok().json(StatusResponse{ is_ok: update_op })
}

/// This function is the API service
/// function for editing the hostname in use
/// on the current Cleo instance. If 
/// the received request and resulting
/// operation are both valid, an instance
/// of the "StatusResponse" as a JSON
/// response is returned. 
#[post("/instance/edit/hostname")]
pub async fn edit_instance_hostname_service(
    payload: Json<UserChangePayload>,
    data: Data<AppData>
) -> HttpResponse {
    let update_op: bool = match edit_instance_hostname(
        &payload.api_token, 
        &payload.new_value,
        &data.pool
    ).await {
        Ok(_op) => true,
        Err(_e) => false
    };
    HttpResponse::Ok().json(StatusResponse{ is_ok: update_op })
}

/// This function is the API service
/// function for editing the IP address
/// of the SMTP server account in use
/// on the current Cleo instance. If 
/// the received request and resulting
/// operation are both valid, an instance
/// of the "StatusResponse" as a JSON
/// response is returned.
#[post("/instance/edit/smtp/server")]
pub async fn edit_smtp_server_service(
    payload: Json<UserChangePayload>,
    data: Data<AppData>
) -> HttpResponse { 
    let update_op: bool = match edit_instance_smtp_server(
        &payload.api_token, 
        &payload.new_value,
        &data.pool
    ).await {
        Ok(_op) => true,
        Err(_e) => false
    };
    HttpResponse::Ok().json(StatusResponse{ is_ok: update_op })
}

/// This function is the API service
/// function for editing the username
/// of the SMTP server account in use
/// on the current Cleo instance. If 
/// the received request and resulting
/// operation are both valid, an instance
/// of the "StatusResponse" as a JSON
/// response is returned.
#[post("/instance/edit/smtp/username")]
pub async fn edit_smtp_username_service(
    payload: Json<UserChangePayload>,
    data: Data<AppData>
) -> HttpResponse {
    let update_op: bool = match edit_smtp_username(
        &payload.api_token, 
        &payload.new_value,
        &data.pool
    ).await {
        Ok(_op) => true,
        Err(_e) => false
    };
    HttpResponse::Ok().json(StatusResponse{ is_ok: update_op })
}

/// This function is the API service
/// function for editing the password
/// of the SMTP server account in use
/// on the current Cleo instance. If 
/// the received request and resulting
/// operation are both valid, an instance
/// of the "StatusResponse" as a JSON
/// response is returned. 
#[post("/instance/edit/smtp/pass")]
pub async fn edit_smtp_password_service(
    payload: Json<UserChangePayload>,
    data: Data<AppData>
) -> HttpResponse { 
    let update_op: bool = match edit_smtp_pass(
        &payload.api_token, 
        &payload.new_value,
        &data.pool
    ).await {
        Ok(_op) => true,
        Err(_e) => false
    };
    HttpResponse::Ok().json(StatusResponse{ is_ok: update_op })
}
