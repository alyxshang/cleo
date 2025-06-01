/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// "Pool" structure
/// to accept multiple
/// connections to a 
/// database.
use sqlx::Pool;

/// Importing the "query"
/// macro to execute SQL
/// queries that return 
/// nothing.
use sqlx::query;

/// Importing the "query_as"
/// macro to execute SQL
/// queries that return 
/// something.
use sqlx::query_as;

/// Importing the "Postgres"
/// to specify the type of
/// database one is connecting
/// to.
use sqlx::postgres::Postgres;

/// Importing the "CleoErr"
/// structure to ctach and handle
/// errors.
use crate::modules::err::CleoErr;

/// Importing the "TimeNow" structure
/// to retrieve the current time.
use crate::modules::utils::TimeNow;

/// Importing the "CleoUser" structure
/// to read and write associated data
/// about Cleo users.
use crate::modules::models::CleoUser;

/// Importing the "UserPost" structure
/// to read and write information about
/// posts written by a user.
use crate::modules::models::UserPost;

/// Importing the function to generate
/// a hash for string.
use crate::modules::utils::hash_string;

/// Importing the "get_user_from_token" 
/// function to retrieve an entry for a Cleo
/// user given their API token.
use crate::modules::db::users::get_user_from_token;

/// This function attempts
/// to create a post for a user.
/// If the operation is successful,
/// an instance of the "UserPost"
/// structure is returned.
pub async fn create_user_post(
    api_token: &String,
    content_type: &String, // "page" or "post"
    content_text: &String, // needs to be html
    pool: &Pool<Postgres>
) -> Result<UserPost, CleoErr>{
    let user_obj: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user_obj) => user_obj,
        Err(e) => return Err::<UserPost, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let content_snippet: String = content_text.chars().take(16).collect();
    let content_id: String = hash_string(
        &format!(
            "{}{}",
            TimeNow::new().to_string(),
            content_snippet
        )
    );
    let post_obj: UserPost = UserPost{
        user_id: user_obj.user_id,
        content_text: content_text.clone(),
        content_type: content_type.to_owned(),
        content_id: content_id.clone(),
    };
    let _insert_op = match query!(
        "INSERT INTO user_posts (user_id, content_text, content_type, content_id) VALUES ($1, $2, $3, $4)",
        post_obj.user_id,
        post_obj.content_text,
        post_obj.content_type,
        post_obj.content_id
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<UserPost, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let post_obj: UserPost = match get_post_by_id(&content_id, pool).await {
        Ok(post_obj) => post_obj,
        Err(e) => return Err::<UserPost, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(post_obj)
}

/// This function attempts
/// to retrieve a post given the
/// post's ID. If the operation 
/// is successful, an instance
/// of the "UserPost" model 
/// is returend. If the operation
/// fails, an error is returned.
pub async fn get_post_by_id(
    content_id: &String,
    pool: &Pool<Postgres>
) -> Result<UserPost, CleoErr> {
    let post_obj: UserPost = match query_as!(
        UserPost,
        "SELECT * FROM user_posts WHERE content_id = $1", 
        content_id
    )
        .fetch_one(pool)
        .await 
    {
        Ok(post_obj) => post_obj,
        Err(e) => return Err::<UserPost, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(post_obj)
}

/// This function attempts
/// to update the text inside
/// a post. If the operation is
/// successful, an empty function
/// is returned. If this operation fails,
/// an error is returned.
pub async fn update_post_text(
    api_token: &String,
    content_id: &String,
    text: &String,
    pool: &Pool<Postgres>
) -> Result<(), CleoErr> {
    let user_obj: CleoUser = match get_user_from_token(&api_token, pool).await {
        Ok(user_obj) => user_obj,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let post_obj: UserPost = match get_post_by_id(&content_id, pool).await {
        Ok(post_obj) => post_obj,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user_obj.user_id == post_obj.user_id {
        let update_op: () = match query!(
            "UPDATE user_posts SET content_text = $1 WHERE content_id = $2", 
            text,
            content_id
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
        let e: String = "Could not verify ownership of the post.".to_string();
        Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    }
}

/// This function attempts to
/// delete a post. If this operation
/// is successful, an empty function
/// is returned. If the operation fails,
/// an error is returned.
pub async fn delete_post(
    api_token: &String,
    content_id: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr> {
    let user_obj: CleoUser = match get_user_from_token(&api_token, pool).await {
        Ok(user_obj) => user_obj,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let post_obj: UserPost = match get_post_by_id(&content_id, pool).await {
        Ok(post_obj) => post_obj,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user_obj.user_id == post_obj.user_id {
        let del_op: () = match query!(
            "DELETE FROM user_posts WHERE content_id = $1", 
            content_id
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
        let e: String = "Could not verify ownership of the post.".to_string();
        Err::<(), CleoErr>(CleoErr::new(&e.to_string()))

    }
}

/// This function attempts
/// to retrieve all of a user's
/// posts. If this operation is succesful,
/// a vector of instances of the "UserPost"
/// structure is returned. If this operation
/// fails, an error is returned.
pub async fn get_user_posts(
    api_token: &String,
    pool: &Pool<Postgres>,
) -> Result<Vec<UserPost>, CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<Vec<UserPost>, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let user_posts: Vec<UserPost> = match query_as!(
        UserPost,
        "SELECT * FROM user_posts WHERE user_id = $1",
        user.user_id
    )   
        .fetch_all(pool)
        .await 
    {
        Ok(user_posts) => user_posts,
        Err(e) => return Err::<Vec<UserPost>, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(user_posts)
}
