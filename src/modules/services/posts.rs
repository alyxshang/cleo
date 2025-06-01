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
/// user posts for explicit
/// typing.
use crate::modules::models::UserPost;

/// Importing the enum to describe
/// all possible types of content.
use crate::modules::payloads::ContentType;

/// Importing the function
/// to delete a post a user
/// has created.
use crate::modules::db::posts::delete_post;

/// Importing the data structure
/// for returning info on whether
/// a write operation was successful
/// or not.
use crate::modules::responses::StatusResponse;

/// Importing the data structure for
/// submitting a payload for updating
/// a post a user has made.
use crate::modules::payloads::UpdatePostPayload;

/// Importing the data structure for
/// submitting a payload for deleting
/// a post a user has made.
use crate::modules::payloads::DeletePostPayload;

/// Importing the function to
/// create a post for a user.
use crate::modules::db::posts::create_user_post;

/// Importing the function to
/// update the text of post.
use crate::modules::db::posts::update_post_text;

/// Importing the data structure for
/// returning information on created
/// posts.
use crate::modules::responses::UserPostResponse;

/// Importing the data structure for
/// submitting a payload for creating
/// a new post.
use crate::modules::payloads::PostCreationPayload;

/// This function is the API service
/// function for creating a post.
/// If the received request and resulting
/// operation are both valid, an instance of
/// the saved post as JSON is returned. 
/// In any other case an error 
/// is returned.
#[post("/posts/create")]
pub async fn create_user_post_service(
    payload: Json<PostCreationPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let content_type: String;
    if &payload.content_type == &ContentType::Page{
        content_type = "page".to_string();
    }
    else {
        content_type = "post".to_string();
    }
    let post: UserPost = match create_user_post(
        &payload.api_token, 
        &content_type, 
        &payload.content_text, 
        &data.pool
    ).await {
        Ok(post) => post,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let resp: UserPostResponse = UserPostResponse{
        content_id: post.content_id,
        content_type: post.content_type,
        user_id: post.user_id,
        content_text: post.content_text
    };
    Ok(HttpResponse::Ok().json(resp))
}

/// This function is the API service
/// function for editing a post of a user.
/// If the received request and resulting
/// operation are both valid, an instance of
/// the "StatusResponse" with a boolean flag
/// is returned as a JSON response. 
#[post("/posts/update")]
pub async fn update_user_post_service(
    payload: Json<UpdatePostPayload>,
    data: Data<AppData>
) -> HttpResponse {
    let update_op: bool = match update_post_text(
        &payload.api_token, 
        &payload.content_id,
        &payload.text, 
        &data.pool
    ).await {
        Ok(_op) => true,
        Err(_e) => false
    };
    HttpResponse::Ok().json(StatusResponse{ is_ok: update_op })
}

/// This function is the API service
/// function for deleting a  post of a user.
/// If the received request and resulting
/// operation are both valid, an instance of
/// the "StatusResponse" with a boolean flag
/// is returned as a JSON response.
#[post("/posts/delete")]
pub async fn delete_user_post_service(
    payload: Json<DeletePostPayload>,
    data: Data<AppData>
) -> HttpResponse {
    let del_op: bool = match delete_post(
        &payload.api_token, 
        &payload.content_id,
        &data.pool
    ).await {
        Ok(_op) => true,
        Err(_e) => false
    };
    HttpResponse::Ok().json(StatusResponse{ is_ok: del_op })
}
