/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Pool"
/// structure to use a pool
/// of connections.
use sqlx::Pool;

/// Importing the "query"
/// macro to execute queries
/// that return nothing.
use sqlx::query;

/// Importing the "query_as"
/// macro to execute queries
/// that return something.
use sqlx::query_as;

/// Importing the "Postgres"
/// structure for explicit 
/// typing.
use sqlx::postgres::Postgres;

/// Importing the "CleoErr"
/// structure to catch and
/// handle errors.
use crate::modules::err::CleoErr;

/// Importing the function to
/// retrieve a post given the 
/// post's ID.
use super::posts::get_post_by_id;

/// Importing the "TimeNow"
/// structure to get the current
/// time.
use crate::modules::utils::TimeNow;

/// Importing the "CleoUser" structure
/// for explicit typing.
use crate::modules::models::CleoUser;

/// Importing the "UserPost" structure
/// for explicit typing.
use crate::modules::models::UserPost;

/// Importing the function to retieve
/// a user given the ID of the user.
use super::tokens::get_user_from_token;

/// Importing the structure representing
/// the model in the database for an extra
/// content field for reading and writing
/// this entity.
use crate::modules::models::ExtraContentField;

/// This function attempts to create an 
/// extra content field for a post with the given
/// data. If the operation is successful, an instance
/// of the "ExtraContentField" structure is returned.
/// If this operation fails, an error is returned.
pub async fn create_extra_field_for_post(
    api_token: &String,
    content_id: &String,
    field_key: &String,
    field_value: &String,
    pool: &Pool<Postgres>,
) -> Result<ExtraContentField, CleoErr> {    
    let post: UserPost = match get_post_by_id(content_id, pool).await {
        Ok(post) => post,
        Err(e) => return Err::<ExtraContentField, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<ExtraContentField, CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.user_id == post.user_id {
        let field_id: String = format!("{}{}", &field_value, TimeNow::new().to_string());
        let extra_field: ExtraContentField = ExtraContentField{
            field_id: field_id.clone(),
            content_id: content_id.to_owned(),
            field_key: field_key.to_owned(),
            field_value: field_value.to_owned()
        };
        let _insert_op = match query!(
            "INSERT INTO extra_content_fields (field_id, content_id, field_key, field_value) VALUES ($1, $2, $3, $4)",
            extra_field.field_id,
            extra_field.content_id,
            extra_field.field_key,
            extra_field.field_value
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<ExtraContentField, CleoErr>(CleoErr::new(&e.to_string()))
        };
        let field_obj: ExtraContentField = match get_extra_field_by_id(&field_id, pool).await {
            Ok(field_obj) => field_obj,
            Err(e) => return Err::<ExtraContentField, CleoErr>(CleoErr::new(&e.to_string()))
        };
        Ok(field_obj)
    }
    else {
        let e: String = format!("Could not verify ownership of token.");
        Err::<ExtraContentField, CleoErr>(CleoErr::new(&e.to_string()))
    }
}

/// This function attempts to retrieve
/// an extra content field belonging to
/// post using the field's ID. If this 
/// operation is successful, an instance
/// of the "ExtraContentField" structure
/// is returned. If this operation fails,
/// an error is returned.
pub async fn get_extra_field_by_id(
    field_id: &String,
    pool: &Pool<Postgres>
) -> Result<ExtraContentField, CleoErr> {
    let field_obj: ExtraContentField = match query_as!(
        ExtraContentField,
        "SELECT * FROM extra_content_fields WHERE field_id = $1", 
        field_id
    )
        .fetch_one(pool)
        .await 
    {
        Ok(field_obj) => field_obj,
        Err(e) => return Err::<ExtraContentField, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(field_obj)
}

/// This function attempts to delete
/// an extra content field given its
/// ID. if the operation is successful,
/// an empty function is returned. If the 
/// operation fails, an error is returned.
pub async fn delete_extra_field_for_post(
    api_token: &String,
    content_id: &String,
    pool: &Pool<Postgres>,
    field_id: &String,
) -> Result<(), CleoErr> {  
    let post: UserPost = match get_post_by_id(content_id, pool).await {
        Ok(post) => post,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.user_id == post.user_id {
        let del_op: () = match query!(
            "DELETE FROM extra_content_fields WHERE field_id = $1", 
            field_id
        )
            .execute(pool)
            .await 
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
        };
        Ok(del_op)
    }
    else {
        let e: String = format!("Could not verify ownership of token.");
        Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    }  
}

/// This function attempts to edit the
/// key of an extra content field in the 
/// database. If this operation is successful,
/// an empty function is returned. If this operation
/// fails, an error is returned.
pub async fn edit_extra_field_key_for_post(
    api_token: &String,
    content_id: &String,
    field_id: &String,
    field_key_new: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr> {    
    let post: UserPost = match get_post_by_id(content_id, pool).await {
        Ok(post) => post,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.user_id == post.user_id {
        let update_op: () = match query!(
            "UPDATE extra_content_fields SET field_key = $1 WHERE field_id = $2", 
            field_key_new,
            field_id
        )
            .execute(pool)
            .await 
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
        };
        Ok(update_op)
    }
    else {
        let e: String = format!("Could not verify ownership of token.");
        Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    } 
}

/// This function attempts to edit the
/// key of an extra content field in the 
/// database. If this operation is successful,
/// an empty function is returned. If this operation
/// fails, an error is returned.
pub async fn edit_extra_field_value_for_post(
    api_token: &String,
    content_id: &String,
    field_id: &String,
    field_value_new: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr> {    
    let post: UserPost = match get_post_by_id(content_id, pool).await {
        Ok(post) => post,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.user_id == post.user_id {
        let update_op: () = match query!(
            "UPDATE extra_content_fields SET field_value = $1 WHERE field_id = $2", 
            field_value_new,
            field_id
        )
            .execute(pool)
            .await 
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
        };
        Ok(update_op)
    }
    else {
        let e: String = format!("Could not verify ownership of token.");
        Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    } 
}
