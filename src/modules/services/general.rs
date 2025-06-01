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

/// Importing the structure to model
/// info about user-uploaded files
/// in the database.
use crate::modules::models::UserFile;

/// Importing the data structure
/// for returning data on all files
/// a user has created.
use crate::modules::responses::UserFiles;

/// Importing the data structure
/// for returning data on all posts
/// a user has created.
use crate::modules::responses::UserPosts;

/// Importing the function to
/// retrieve all the files 
/// a user has uploaded.
use crate::modules::db::files::get_user_files;

/// Importing the function to
/// retrieve all the posts a 
/// user has made.
use crate::modules::db::posts::get_user_posts;

/// Importing the structure to
/// return info on user files.
use crate::modules::responses::UserFileResponse;

/// Importing the structure for 
/// submitting payloads for actions
/// that only require an API token.
use crate::modules::payloads::TokenOnlyPayload;

/// Importing the structure for modelling instance
/// information in the database for explicit
/// typing.
use crate::modules::models::InstanceInformation;

/// Importing the data structure for
/// returning information on created
/// posts.
use crate::modules::responses::UserPostResponse;

/// Importing the function to
/// retrieve instance information
/// from the database.
use crate::modules::db::general::get_instance_info;

/// This function is the API
/// service function for retrieving
/// a vector of all posts made by a
/// user.If the received request and resulting
/// operation are both valid, a vector with
/// instances of the "UserPostResponse" structure
/// is returned. In any other case an error is
/// returned.
#[post("/posts/all")]
pub async fn get_user_posts_service(
    payload: Json<TokenOnlyPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut sanitized: Vec<UserPostResponse> = Vec::new();
    let posts: Vec<UserPost> = match get_user_posts(&payload.api_token, &data.pool).await {
        Ok(users) => users,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    for post in posts {
        let resp_post: UserPostResponse = UserPostResponse{
            content_id: post.content_id,
            content_type: post.content_type,
            user_id: post.user_id,
            content_text: post.content_text
        };
        sanitized.push(resp_post);
    }
    let resp: UserPosts = UserPosts{ posts: sanitized };
    Ok(HttpResponse::Ok().json(resp))
}

/// This function contains the
/// service functions that fetches
/// information on all files a user
/// has uploaded. If this operation fails,
/// an error is returned.
#[post("/files/all")]
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
           file_id: file.file_id,
           file_url: format!("{}{}", &info.hostname, &file.file_path),
           file_name: file.file_path
        };
        sanitized.push(resp_file);
    }
    let resp: UserFiles = UserFiles{ files: sanitized };
    Ok(HttpResponse::Ok().json(resp))
}
