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
/// user API keys for explicit
/// typing.
use crate::modules::models::UserAPIToken;

/// Importing the function to
/// delete an API token.
use crate::modules::db::tokens::delete_token;

/// Importing the data structure
/// for returning info on whether
/// a write operation was successful
/// or not.
use crate::modules::responses::StatusResponse;

/// Importing the data structure for
/// submitting a payload that contains
/// info for deleting an API token.
use crate::modules::payloads::DelTokenPayload;

/// Importing the data structure for
/// returning information a created 
/// API token.
use crate::modules::responses::APITokenResponse;

/// Importing the structure for a 
/// payload to carry out actions
/// requiring higher privileges.
use crate::modules::payloads::AuthActionPayload;

/// Importing the function to create
/// an API token for a Cleo user.
use crate::modules::db::tokens::create_api_token_for_user;

/// This function is the API service
/// function for creating an API token
/// for a user.If the received request 
/// and resulting operation are both valid, 
/// an instance of "APITokenResponse" 
/// structure as a JSON response is
/// returned. In any other case an 
/// error is returned.
#[post("/token/create")]
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

/// This function is the API service
/// function for deleting an API
/// token for a user. If the received 
/// request and resulting operation are 
/// both valid, an instance of the "StatusResponse" 
/// with a boolean flag is returned as a 
/// JSON response. 
#[post("/token/delete")]
pub async fn delete_api_token_service(
    payload: Json<DelTokenPayload>,
    data: Data<AppData>
) -> HttpResponse {
    let del_op: bool = match delete_token(
        &payload.token, 
        &payload.username,
        &payload.password, 
        &data.pool
    ).await {
        Ok(_op) => true,
        Err(_e) => false
    };
    HttpResponse::Ok().json(StatusResponse{ is_ok: del_op })
}
