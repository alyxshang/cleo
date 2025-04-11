/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

use sqlx::Pool;
use sqlx::query;
use bcrypt::hash;
use bcrypt::verify;
use sqlx::query_as;
use super::err::CleoErr;
use bcrypt::DEFAULT_COST;
use super::utils::TimeNow;
use super::models::UserKey;
use super::models::UserFile;
use super::models::CleoUser;
use super::models::UserPost;
use sqlx::postgres::Postgres;
use super::utils::hash_string;
use super::utils::generate_key;
use super::models::UserAPIToken;
use super::models::ExtraContentField;
use super::models::InstanceInformation;

// Done. Has service.
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

// Done. Has service.
pub async fn get_user_files(
    api_token: &String,
    pool: &Pool<Postgres>,
) -> Result<Vec<UserFile>, CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<Vec<UserFile>, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let user_files: Vec<UserFile> = match query_as!(
        UserFile,
        "SELECT * FROM user_files WHERE user_id = $1",
        user.user_id
    )   
        .fetch_all(pool)
        .await 
    {
        Ok(user_files) => user_files,
        Err(e) => return Err::<Vec<UserFile>, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(user_files)
}

// Done. // Has service.
pub async fn get_instance_info(
    pool: &Pool<Postgres>
) -> Result<InstanceInformation, CleoErr>{
    let info: InstanceInformation = match query_as!(
        InstanceInformation,
        "SELECT * FROM instance_info",
    )   
        .fetch_one(pool)
        .await 
    {
        Ok(info) => info,
        Err(e) => return Err::<InstanceInformation, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(info)
}
