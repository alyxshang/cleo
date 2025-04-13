/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "get"
/// decorator to make a service
/// that accepts "GET" requests.
use actix_web::get;

/// Importing the "Result"
/// enum for Actix Web services.
use actix_web::Result;

/// Importing the "Data"
/// structure to register
/// persistent app data.
use actix_web::web::Data;

/// Importing the "Path"
/// structure from Actix  Web
/// to extract data.
use actix_web::web::Path;

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
/// email tokens for explicit
/// typing.
use crate::modules::models::EmailToken;

/// Importing the function to change the
/// verification status of a user.
use crate::modules::db::users::set_verified;

/// Importing the data structure
/// for returning info on whether
/// a write operation was successful
/// or not.
use crate::modules::responses::StatusResponse;

/// Importing the structure to extract
/// data from an URL.
use crate::modules::extractors::TokenExtractor;

/// Importing the function to delete an email token
/// object from the database.
use crate::modules::db::email::delete_email_token;

/// Importing the function to retrieve an email
/// token object from the database given the token.
use crate::modules::db::email::get_object_from_token;


/// This function is the API service
/// function for verifying an email token.
/// If the received request and resulting
/// operation are both valid, an instance of
/// the "StatusResponse" structure as JSON
/// is returned with a boolean flag. In any 
/// other case an error is returned.
#[get("/email/{token}")]
pub async fn verify_email_service(
    token: Path<TokenExtractor>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr>{
    let token: String = token.token.clone();
    let token_obj: EmailToken = match get_object_from_token(&token, &data.pool).await{
        Ok(token_obj) => token_obj,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let del_op: bool = match delete_email_token(&token_obj.email_token, &data.pool).await {
        Ok(_op) => true,
        Err(_e) => false
    };
    if del_op{
        let result: bool  = match set_verified(&token_obj.user_id, &data.pool).await {
            Ok(_op) => true,
            Err(_e) => false
        };  
        Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))    }
    else {
        let e: String = format!("The token \"{}\" could not be verified.", &token_obj.email_token);
        return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()));

    }
}

