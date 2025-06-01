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

/// Importing the strcuture for modelling
/// extra content fields in the database.
use crate::modules::models::ExtraContentField;

/// Importing the data structure
/// for returning info on whether
/// a write operation was successful
/// or not.
use crate::modules::responses::StatusResponse;

/// Importing the data structure for returning
/// information on an extra content field that
/// has been created.
use crate::modules::responses::ExtraContentFieldResponse;

/// Importing the function to create an
/// extra content field for a post.
use crate::modules::db::ecf::create_extra_field_for_post;

/// Importing the function to delete an
/// extra content field for a post.
use crate::modules::db::ecf::delete_extra_field_for_post;

/// Importing the function to edit the key
/// of an extra content field of a post.
use crate::modules::db::ecf::edit_extra_field_key_for_post;

/// Importing the data structure for submitting
/// payload for editing an extra content 
/// field.
use crate::modules::payloads::EditExtraContentFieldPayload;

/// Importing the function to edit the value
/// of an extra content field of a post.
use crate::modules::db::ecf::edit_extra_field_value_for_post;

/// Importing the data structure for submitting
/// a payload for deleting an extra content 
/// field.
use crate::modules::payloads::DeleteExtraContentFieldPayload;

/// Importing the data structure for submitting
/// a payload for creating an extra content 
/// field.
use crate::modules::payloads::ExtraContentFieldCreationPayload;

/// This function is the API service
/// function for creating an extra
/// content field on a post of a user.
/// If the received request and resulting
/// operation are both valid, an instance of
/// the saved extra content field as JSON
/// is returned. In any other case an error 
/// is returned.
#[post("/ecf/create")]
pub async fn create_extra_content_field_service(
    payload: Json<ExtraContentFieldCreationPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let field: ExtraContentField = match create_extra_field_for_post(
        &payload.api_token, 
        &payload.content_id, 
        &payload.field_key, 
        &payload.field_value, 
        &data.pool
    ).await {
        Ok(field) => field,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let resp: ExtraContentFieldResponse = ExtraContentFieldResponse {
        field_id: field.field_id,
        content_id: field.content_id,
        field_key: field.field_key,
        field_value: field.field_value
    };
    Ok(HttpResponse::Ok().json(resp))
}

/// This function is the API service
/// function for editing an extra
/// content field's key on a post of a user.
/// If the received request and resulting
/// operation are both valid, an instance of
/// the "StatusResponse" with a boolean flag
/// is returned as a JSON response. 
#[post("/ecf/edit/key")]
pub async fn edit_extra_content_field_key_service(
    payload: Json<EditExtraContentFieldPayload>,
    data: Data<AppData>
) -> HttpResponse {
    let update_op: bool = match edit_extra_field_key_for_post(
        &payload.api_token, 
        &payload.content_id,
        &payload.field_id,
        &payload.new_value,
        &data.pool
    ).await {
        Ok(_op) => true,
        Err(_e) => false
    };
    HttpResponse::Ok().json(StatusResponse{ is_ok: update_op })
}

/// This function is the API service
/// function for editing an extra
/// content field's value on a post of a user.
/// If the received request and resulting
/// operation are both valid, an instance of
/// the "StatusResponse" with a boolean flag
/// is returned as a JSON response.
#[post("/ecf/edit/value")]
pub async fn edit_extra_content_field_value_service(
    payload: Json<EditExtraContentFieldPayload>,
    data: Data<AppData>
) -> HttpResponse {
    let del_op: bool = match edit_extra_field_value_for_post(
        &payload.api_token, 
        &payload.content_id,
        &payload.field_id,
        &payload.new_value,
        &data.pool
    ).await {
        Ok(_op) => true,
        Err(_e) => false
    };
    HttpResponse::Ok().json(StatusResponse{ is_ok: del_op })
}

/// This function is the API service
/// function for deleting an extra
/// content field on a post of a user.
/// If the received request and resulting
/// operation are both valid, an instance of
/// the "StatusResponse" with a boolean flag
/// is returned as a JSON response.
#[post("/ecf/delete")]
pub async fn delete_extra_content_field_service(
    payload: Json<DeleteExtraContentFieldPayload>,
    data: Data<AppData>
) -> HttpResponse {
    let del_op: bool = match delete_extra_field_for_post(
        &payload.api_token, 
        &payload.content_id,
        &data.pool,
        &payload.field_id
    ).await {
        Ok(_op) => true,
        Err(_e) => false
    };
    HttpResponse::Ok().json(StatusResponse{ is_ok: del_op })
}
