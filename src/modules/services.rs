/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "get"
/// decorator to make a service
/// that accepts "GET" requests.
use actix_web::get;

use actix_web::http::header::map::Keys;
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

use crate::ExtraContentField;

/// Importing this crate's
/// error structure.
use super::err::CleoErr;

/// Importing the "AppData"
/// structure to register
/// persistent app data.
use super::units::AppData;

/// Importing the function
/// to update a user's
/// profile picture.
use super::rw::update_pfp;

/// Importing the function
/// to create a new user.
use super::rw::create_user;

/// Importing the model for
/// user keys for explicit
/// typing.
use super::models::UserKey;

/// Importing the function
/// to delete a post a user
/// has created.
use super::rw::delete_post;

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
use super::payloads::UserCreationPayload;

/// Importing the data structure for
/// submitting a payload for creating
/// a new post.
use super::payloads::PostCreationPayload;

/// Importing the function to update
/// the address of the instance's SMTP
/// server.
use super::rw::edit_instance_smtp_server;

/// Importing the function to create an
/// extra content field for a post.
use super::rw::create_extra_field_for_post;

/// Importing the function to delete an
/// extra content field for a post.
use super::rw::delete_extra_field_for_post;

/// Importing the data structure for submitting
/// a payload for deleting a user key.
use super::payloads::UserKeyDeletionPayload;

/// Importing the structure to return info on
/// all types of users present on an instance.
use super::responses::InstanceUsersResponse;

/// Importing the data structure for submitting
/// a payload for deleting a file a user has
/// uploaded.
use super::payloads::DeleteUserFilePayload;

/// Importing the data structure for
/// returning information on created
/// users.
use super::responses::UserCreationResponse;

/// Importing the function to edit the key
/// of an extra content field of a post.
use super::rw::edit_extra_field_key_for_post;

/// Importing the structure for returning
/// information on a created user key.
use super::responses::UserKeyCreationResponse;

/// Importing the function to edit the value
/// of an extra content field of a post.
use super::rw::edit_extra_field_value_for_post;

/// Importing the data structure for returning
/// information on an extra content field that
/// has been created.
use super::responses::ExtraContentFieldResponse;

/// Importing the data structure for submitting
/// payload for editing an extra content 
/// field.
use super::payloads::EditExtraContentFieldPayload;

/// Importing the data structure for submitting
/// a payload for deleting an extra content 
/// field.
use super::payloads::DeleteExtraContentFieldPayload;

/// Importing the data structure for submitting
/// a payload for creating an extra content 
/// field.
use super::payloads::ExtraContentFieldCreationPayload;

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

/* TODO */
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


#[post("/posts/update")]
pub async fn update_user_post_service(
    payload: Json<UpdatePostPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut result: bool = false;
    let _update_op: () = match update_post_text(
        &payload.api_token, 
        &payload.content_id,
        &payload.text, 
        &data.pool
    ).await {
        Ok(_update_op) => {result = true},
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))
}

#[post("/posts/delete")]
pub async fn delete_user_post_service(
    payload: Json<DeletePostPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut result: bool = false;
    let _del_op: () = match delete_post(
        &payload.api_token, 
        &payload.content_id,
        &data.pool
    ).await {
        Ok(_del_op) => {result = true},
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))
}

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

#[post("/ecf/edit/key")]
pub async fn edit_extra_content_field_key_service(
    payload: Json<EditExtraContentFieldPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut result: bool = false;
    let _del_op: () = match edit_extra_field_key_for_post(
        &payload.api_token, 
        &payload.content_id,
        &payload.field_id,
        &payload.new_value,
        &data.pool
    ).await {
        Ok(_del_op) => {result = true},
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))
}

#[post("/ecf/edit/value")]
pub async fn edit_extra_content_field_value_service(
    payload: Json<EditExtraContentFieldPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut result: bool = false;
    let _del_op: () = match edit_extra_field_value_for_post(
        &payload.api_token, 
        &payload.content_id,
        &payload.field_id,
        &payload.new_value,
        &data.pool
    ).await {
        Ok(_del_op) => {result = true},
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))
}

#[post("/ecf/delete")]
pub async fn delete_extra_content_field_service(
    payload: Json<DeleteExtraContentFieldPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut result: bool = false;
    let _del_op: () = match delete_extra_field_for_post(
        &payload.api_token, 
        &payload.content_id,
        &data.pool,
        &payload.field_id
    ).await {
        Ok(_del_op) => {result = true},
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))
}

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

#[post("/user/keys/create")]
pub async fn create_user_key_service(
    payload: Json<UserKeyPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let key: UserKey = match create_user_key(
        &payload.key_type, 
        &payload.api_token, 
        &data.pool
    ).await {
        Ok(key) => key,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let resp: UserKeyCreationResponse = UserKeyCreationResponse{
        key_type: key.key_type,
        user_key: key.user_key
    };
    Ok(HttpResponse::Ok().json(resp))
}

#[post("/user/keys/delete")]
pub async fn delete_user_key_service(
    payload: Json<UserKeyDeletionPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut result: bool = false;
    let _del_op: () = match delete_user_key(
        &payload.api_token, 
        &payload.key_id,
        &data.pool
    ).await {
        Ok(_del_op) => {result = true},
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))
}

#[get("/info/admins")]
pub async fn get_instance_admins_service(
    payload: Json<TokenOnlyPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut sanitized: Vec<UserCreationResponse> = Vec::new();
    let users: Vec<CleoUser> = match get_instance_users(&payload.api_token, &data.pool).await {
        Ok(users) => users,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    for user in users {
        let resp_user: UserCreationResponse = UserCreationResponse{
            user_id: user.user_id,
            display_name: user.display_name,
            is_verified: user.is_verified,
            username: user.username,
            email_addr: user.email_addr,
            pfp_url: user.pfp_url,
            is_admin: user.is_admin
        };
        sanitized.push(resp_user);
    }
    let resp: InstanceUsersResponse = InstanceUsersResponse{ users: sanitized };
    Ok(HttpResponse::Ok().json(resp))
}

#[get("/info/users")]
pub async fn get_instance_users_service(
    payload: Json<TokenOnlyPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut sanitized: Vec<UserCreationResponse> = Vec::new();
    let users: Vec<CleoUser> = match get_instance_users(&payload.api_token, &data.pool).await {
        Ok(users) => users,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    for user in users {
        let resp_user: UserCreationResponse = UserCreationResponse{
            user_id: user.user_id,
            display_name: user.display_name,
            is_verified: user.is_verified,
            username: user.username,
            email_addr: user.email_addr,
            pfp_url: user.pfp_url,
            is_admin: user.is_admin
        };
        sanitized.push(resp_user);
    }
    let resp: InstanceUsersResponse = InstanceUsersResponse{ users: sanitized };
    Ok(HttpResponse::Ok().json(resp))
}

#[get("/info/keys")]
pub async fn get_user_keys_service(
    payload: Json<TokenOnlyPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut sanitized: Vec<UserKeyCreationResponse> = Vec::new();
    let keys: Vec<UserKey> = match get_user_keys(&payload.api_token, &data.pool).await {
        Ok(keys) => keys,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    for key in keys {
        let resp_key: UserKeyCreationResponse = UserKeyCreationResponse{
           key_type: key.key_type,
           user_key: key.user_key
        };
        sanitized.push(resp_key);
    }
    let resp: UserKeys = UserKeys{ keys: sanitized };
    Ok(HttpResponse::Ok().json(resp))
}

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

#[get("/info/posts")]
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

#[get("/info/instance")]
pub async fn get_instance_info_service(
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let info: InstanceInformation = match get_instance_info(&data.pool).await{
        Ok(info) => info,
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let resp: InstanceResponse = InstanceResponse{
        name: info.instance_name,
        hostname: info.hostname
    };
    Ok(HttpResponse::Ok().json(resp))
}

#[post("/instance/edit/name")]
pub async fn edit_instance_name_service(
    payload: Json<UserChangePayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut result: bool = false;
    let _del_op: () = match edit_instance_name(
        &payload.api_token, 
        &payload.new_value,
        &data.pool
    ).await {
        Ok(_del_op) => {result = true},
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))
}

#[post("/instance/edit/hostname")]
pub async fn edit_instance_hostname_service(
    payload: Json<UserChangePayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut result: bool = false;
    let _del_op: () = match edit_instance_hostname(
        &payload.api_token, 
        &payload.new_value,
        &data.pool
    ).await {
        Ok(_del_op) => {result = true},
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))
}

#[post("/instance/edit/smtp/server")]
pub async fn edit_smtp_server_service(
    payload: Json<UserChangePayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut result: bool = false;
    let _del_op: () = match edit_instance_smtp_server(
        &payload.api_token, 
        &payload.new_value,
        &data.pool
    ).await {
        Ok(_del_op) => {result = true},
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))
}

#[post("/instance/edit/smtp/username")]
pub async fn edit_smtp_username_service(
    payload: Json<UserChangePayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut result: bool = false;
    let _del_op: () = match edit_smtp_username(
        &payload.api_token, 
        &payload.new_value,
        &data.pool
    ).await {
        Ok(_del_op) => {result = true},
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))
}

#[post("/instance/edit/smtp/pass")]
pub async fn edit_smtp_password_service(
    payload: Json<UserChangePayload>,
    data: Data<AppData>
) -> Result<HttpResponse, CleoErr> {
    let mut result: bool = false;
    let _del_op: () = match edit_smtp_pass(
        &payload.api_token, 
        &payload.new_value,
        &data.pool
    ).await {
        Ok(_del_op) => {result = true},
        Err(e) => return Err::<HttpResponse, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(StatusResponse{ is_ok: result }))
}