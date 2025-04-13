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

use crate::modules::models::UserKey;

/// Importing the "AppData"
/// structure to register
/// persistent app data.
use crate::modules::units::AppData;

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
        let created: UserKey = match create_user_key(&("admin".to_string()), &payload.api_token, &data.pool).await {
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
        let created: UserKey = match create_user_key(&("normal".to_string()), &payload.api_token, &data.pool).await {
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
/// is returned as a JSON response. In any other
/// case an error is returned.
#[post("/keys/delete")]
pub async fn delete_user_key_service(
    payload: Json<UserKeyDeletionPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut result: bool = false;
    let _del_op: () = match delete_user_key(
        &payload.api_token, 
        &payload.key_id,
        &data.pool,
    ).await {
        Ok(_del_op) => {result = true},
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))
}
