/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Pool"
/// structure to accept
/// multiple connections
/// to a Postgres database.
use sqlx::Pool;

/// Importing the
/// "query" macro to
/// execute SQL queries
/// that return nothing.
use sqlx::query;

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

/// Importing the "TimeNow" structure
/// to get the current time.
use crate::modules::utils::TimeNow;

/// Importing the "UserFile" structure
/// to read and write information about
/// user-uploaded files.
use crate::modules::models::UserFile;

/// Importing the "CleoUser" structure
/// to read and write information about
/// a Cleo user.
use crate::modules::models::CleoUser;

/// Importing the "get_user_from_token" 
/// function to retrieve a user using
/// their API token.
use crate::modules::db::users::get_user_from_token;

/// This function attempts to
/// save the path of an uploaded
/// file for a user in the database.
/// If this operation is successful,
/// an instance of the "UserFile" model
/// is returned. If this operation fails
/// an error is returned.
pub async fn create_user_file(
    api_token: &String,
    file_path: &String,
    file_url: &String,
    pool: &Pool<Postgres>,
) -> Result<UserFile, CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<UserFile, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let file_id: String = format!("{}{}", TimeNow::new().to_string(), file_path);
    let user_file_obj: UserFile = UserFile{
        file_id: file_id.clone(),
        user_id: user.user_id,
        file_path: file_path.clone(),
        file_url: file_url.clone()
    };
    let _insert_op = match query!(
        "INSERT INTO user_files (file_id, user_id, file_path, file_url) VALUES ($1, $2, $3, $4)",
        user_file_obj.file_id,
        user_file_obj.user_id,
        user_file_obj.file_path,
        user_file_obj.file_url
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<UserFile, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let file_obj: UserFile = match get_file_by_id(&file_id, pool).await {
        Ok(file_obj) => file_obj,
        Err(e) => return Err::<UserFile, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(file_obj)
}

/// This function attempts to 
/// retrieve an instance of the
/// "UserFile" model given the file's
/// ID. If the operation is successful,
/// an instance of the "UserFile" model
/// is returned. If this operation fails,
/// an error is returned.
pub async fn get_file_by_id(
    file_id: &String,
    pool: &Pool<Postgres>,
) -> Result<UserFile, CleoErr>{
    let file_obj: UserFile = match query_as!(
        UserFile,
        "SELECT * FROM user_files WHERE file_id = $1", 
        file_id
    )
        .fetch_one(pool)
        .await 
    {
        Ok(file_obj) => file_obj,
        Err(e) => return Err::<UserFile, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(file_obj)
}

/// This function attempts to 
/// delete a record for a file
/// uploaded by a user. If this
/// operation fails, an error is 
/// returned. If the operation is
/// successful, an empty function
/// is returned.
pub async fn delete_user_file(
    api_token: &String,
    file_id: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr> {
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let file_obj: UserFile = match get_file_by_id(&file_id, pool).await {
        Ok(file_obj) => file_obj,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.user_id == file_obj.file_id {
        let del_op: () = match query!(
            "DELETE FROM user_files WHERE file_id = $1", 
            file_id
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
        let e: String = format!("Could not verify ownership of the file.");
        Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    } 
}

/// This function attempts
/// to retrieve a list of 
/// all records for files
/// uploaded by a user given
/// a user's API token. If this
/// operation is successful,
/// a vector of instances of
/// the "UserFile" model is returned.
/// If this operation fails, an error
/// is returned.
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
