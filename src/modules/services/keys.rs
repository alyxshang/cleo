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

/// Importing the structure for
/// modelling user keys in the
/// database.
use crate::modules::models::UserKey;

/// Importing the "AppData"
/// structure to register
/// persistent app data.
use crate::modules::units::AppData;

/// Importing the function to retrieve
/// all user keys an administrator has
/// created.
use crate::modules::db::keys::get_user_keys;

/// Importing the structure for submitting
/// a payload in a POST request for creating
/// a user key.
use crate::modules::payloads::UserKeyPayload;

/// Importing the data structure
/// for returning info on whether
/// a write operation was successful
/// or not.
use crate::modules::responses::StatusResponse;

/// Importing the function for writing
/// a new user key to the database,
use crate::modules::db::keys::create_user_key;

/// Importing the function for deleting
/// a user key from the database.
use crate::modules::db::keys::delete_user_key;

/// Importing the structure for 
/// submitting payloads for actions
/// that only require an API token.
use crate::modules::payloads::TokenOnlyPayload;

/// Importing the structure for returning
/// all user keys an administrator has created
/// as a JSON response.
use crate::modules::responses::UserKeysResponse;

/// Importing the structure for submitting
/// a payload in a POST request for deleting
/// a user key.
use crate::modules::payloads::UserKeyDeletionPayload;

/// Importing the structure for returning information
/// about a new user key as a JSON response.
use crate::modules::responses::UserKeyCreationResponse;

/// This function is the API service
/// function for creating a user key.
/// If the received request and resulting
/// operation are both valid, an instance of
/// the user key for a user as JSON
/// is returned. In any other case an error 
/// is returned.
#[post("/keys/create")]
pub async fn create_user_key_service(
    payload: Json<UserKeyPayload>,
    data: Data<AppData> 
) -> Result<HttpResponse, CleoErr>{
    if payload.key_type == "admin"{
        let created: UserKey = match create_user_key(&payload.username, &("admin".to_string()), &payload.api_token, &data.pool).await {
            Ok(created) => created,
            Err(e) =>return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
        };
        let resp: UserKeyCreationResponse = UserKeyCreationResponse{
            key_type: "admin".to_string(),
            user_key: created.user_key
        };
        Ok(HttpResponse::Ok().json(resp))
    }
    else if payload.key_type == "normal"{
        let created: UserKey = match create_user_key(&payload.username, &("normal".to_string()), &payload.api_token, &data.pool).await {
            Ok(created) => created,
            Err(e) =>return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
        };
        let resp: UserKeyCreationResponse = UserKeyCreationResponse{
            key_type: "normal".to_string(),
            user_key: created.user_key
        };
        Ok(HttpResponse::Ok().json(resp))
    }
    else {
        let e: String = format!("\"{}\" is not a valid key type.", &payload.key_type);
        return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()));
    }
}

/// This function is the API service
/// function for deleting an extra
/// content field on a post of a user.
/// If the received request and resulting
/// operation are both valid, an instance of
/// the "StatusResponse" with a boolean flag
/// is returned as a JSON response. 
#[post("/keys/delete")]
pub async fn delete_user_key_service(
    payload: Json<UserKeyDeletionPayload>,
    data: Data<AppData>
) -> HttpResponse {
    let del_op: bool = match delete_user_key(
        &payload.api_token, 
        &payload.key_id,
        &data.pool,
    ).await {
        Ok(_del_op) => true,
        Err(_e) => false
    };
    HttpResponse::Ok().json(StatusResponse{ is_ok: del_op })
}

/// This function is the API service
/// function for retrieving all user
/// keys an administrator has created.
/// If the received request and resulting
/// operation are both valid, an instance
/// of the "UserKeysResponse" structure as
/// a JSON response is returned. In any other 
/// case an error is returned.
#[post("/keys/all")]
pub async fn get_user_keys_service(
    payload: Json<TokenOnlyPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr>{
    let keys: Vec<UserKey> = match get_user_keys(&payload.api_token, &data.pool).await {
        Ok(keys) => keys,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let mut result: Vec<UserKeyCreationResponse> = Vec::new();
    for key in keys {
        let created: UserKeyCreationResponse = UserKeyCreationResponse{
            key_type: key.key_type,
            user_key: key.user_key
        };
        result.push(created);
    }
    Ok(HttpResponse::Ok().json(UserKeysResponse{ keys: result }))
}
