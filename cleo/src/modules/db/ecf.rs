/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

use sqlx::Pool;
use sqlx::query;
use sqlx::query_as;
use sqlx::postgres::Postgres;
use crate::modules::err::CleoErr;
use crate::modules::utils::TimeNow;
use crate::modules::models::CleoUser;
use crate::modules::models::UserPost;
use crate::modules::models::ExtraContentField;

// Done. Has service.
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

// Done.
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

// Done. Has service.
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

// Done. Has service.
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

// Done. Has service.
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
