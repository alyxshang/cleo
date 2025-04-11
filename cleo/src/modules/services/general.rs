/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "get"
/// decorator to make a service
/// that accepts "GET" requests.
use actix_web::get;

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

use super::rw::get_user_from_email_token;

use super::rw::set_verified;

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
