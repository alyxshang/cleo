/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

use sqlx::Pool;
use sqlx::query;
use bcrypt::hash;
use bcrypt::verify;
use sqlx::query_as;
use crate::modules::err::CleoErr;
use bcrypt::DEFAULT_COST;
use crate::modules::utils::TimeNow;
use crate::modules::models::UserFile;
use crate::moduls::models::CleoUser;
use super::models::UserPost;
use sqlx::postgres::Postgres;
use super::utils::hash_string;
use super::utils::generate_key;
use super::models::UserAPIToken;

// Done. PENDING.
pub async fn create_user_file(
    api_token: &String,
    file_path: &String,
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
        file_path: file_path.clone()
    };
    let _insert_op = match query!(
        "INSERT INTO user_files (file_id, user_id, file_path) VALUES ($1, $2, $3)",
        user_file_obj.file_id,
        user_file_obj.user_id,
        user_file_obj.file_path
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

// Done.
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

// Done. has service.
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
