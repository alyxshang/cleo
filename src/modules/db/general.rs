/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Pool"
/// structure to accept 
/// multiple connections
/// to the database at the
/// same time.
use sqlx::Pool;

/// Importing the
/// "query_as" macro to
/// execute SQL queries
/// that return something.
use sqlx::query_as;

/// Importing the "Postgres"
/// structure to specify the 
/// database one is connecting to.
use sqlx::postgres::Postgres;

/// Importing the "CleoErr" structure
/// to catch and handle errors.
use crate::modules::err::CleoErr;

/// Importing the "UserFile" structure
/// to read and write information about
/// user-uploaded files.
use crate::modules::models::UserFile;

/// Importing the "CleoUser" structure
/// to read and write information about
/// a Cleo user.
use crate::modules::models::CleoUser;

/// Importing the "UserPost" structure
/// to read and write information about
/// posts written by a user.
use crate::modules::models::UserPost;

/// Importing the "InstanceInformation"
/// structure to write information about
/// the instance to the database.
use crate::modules::models::InstanceInformation;

/// Importing the "get_user_from_token" 
/// function to retrieve an entry for a Cleo
/// user given their API token.
use crate::modules::db::users::get_user_from_token;

/// This function attempts to fetch
/// all posts a user has made. if the operation
/// is successful, a vector of instances of the
/// "UserPost" structure is returned. If the operation
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

/// This function attempts to fetch
/// all files a user has uploaded. if the operation
/// is successful, a vector of instances of the
/// "UserFile" structure is returned. If the operation
/// fails, an error is returned.
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

/// This function attempts to fetch
/// the information on a Cleo instance.
/// If the operation fails, an error is 
/// returned.
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
