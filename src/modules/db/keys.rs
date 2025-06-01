/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Pool"
/// structure to use a pool
/// of connections.
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

/// Importing the "CleoErr"
/// structure to catch and
/// handle errors.
use crate::modules::err::CleoErr;

/// Importing the "TimeNow"
/// structure to get the current
/// time.
use crate::modules::utils::TimeNow;

/// Importing the "UserKey" structure
/// to read and write information about
/// user keys from and to the database.
use crate::modules::models::UserKey;

/// Importing the "CleoUser" structure
/// to read and write information about
/// a Cleo user.
use crate::modules::models::CleoUser;

/// Importing the function to generate
/// a user key.
use crate::modules::utils::generate_key;

/// Importing the function to retieve
/// a user given the ID of the user.
use crate::modules::db::users::get_user_from_token;

/// This function attempts to
/// create a user key. If this operation
/// is successful an instance of the "UserKey"
/// structure is returned. if this operation
/// fails, an error is returned.
pub async fn create_user_key(
    username: &String,
    key_type: &String,
    api_token: &String,
    pool: &Pool<Postgres>,
) -> Result<UserKey, CleoErr> {
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<UserKey, CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.is_admin{
        let user_key: String;
        if key_type == &("admin".to_string()){
            user_key = match generate_key(&16){
                Ok(user_key) => user_key,
                Err(e) => return Err::<UserKey, CleoErr>(CleoErr::new(&e.to_string()))
            };        
        }
        else if key_type == &("normal".to_string()){
            user_key = match generate_key(&10){
                Ok(user_key) => user_key,
                Err(e) => return Err::<UserKey, CleoErr>(CleoErr::new(&e.to_string()))
            };        
        }
        else {
            let e: String = format!(
                "\"{}\" is not a valid user key type.",
                &key_type
            );
            return Err::<UserKey, CleoErr>(CleoErr::new(&e.to_string()))
        }
        let key_id: String = format!("{}{}", &user_key, TimeNow::new().to_string());
        let user_key_obj: UserKey = UserKey{
            key_id: key_id.clone(),
            user_id: user.user_id,
            user_key: user_key,
            key_type: key_type.to_owned(),
            key_used: false,
            username: username.to_string()
        };
        let _insert_op = match query!(
            "INSERT INTO user_keys (key_id, user_id, user_key, key_type, key_used, username) VALUES ($1, $2, $3, $4, $5, $6)",
            user_key_obj.key_id,
            user_key_obj.user_id,
            user_key_obj.user_key,
            user_key_obj.key_type,
            user_key_obj.key_used,
            user_key_obj.username
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<UserKey, CleoErr>(CleoErr::new(&e.to_string()))
        };
        let user_key_obj_w: UserKey = match get_user_key_by_id(&key_id, pool).await {
            Ok(user_key_obj_w) => user_key_obj_w,
            Err(e) => return Err::<UserKey, CleoErr>(CleoErr::new(&e.to_string()))
        };
        Ok(user_key_obj_w)
    }
    else {
        let e: String = format!("User is not an administrator.");
        Err::<UserKey, CleoErr>(CleoErr::new(&e.to_string()))
    }
}

/// This function attempts to
/// retrieve an instance of a
/// user key given the key's ID.
/// If this operation fails, an
/// error is returned.
pub async fn get_user_key_by_id(
    user_key_id: &String,
    pool: &Pool<Postgres>,
) -> Result<UserKey, CleoErr>{
    let user_key_obj: UserKey = match query_as!(
        UserKey,
        "SELECT * FROM user_keys WHERE key_id = $1", 
        user_key_id
    )   
        .fetch_one(pool)
        .await 
    {
        Ok(user_key_obj) => user_key_obj,
        Err(e) => return Err::<UserKey, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(user_key_obj)
}

/// This function attempts to 
/// delete a user key from the
/// database. if this operation
/// is successful, an empty
/// function is returned. If this
/// operation fails, an error is 
/// returned.
pub async fn delete_user_key(
    api_token: &String,
    key_id: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr> {
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.is_admin {
        let del_op: () = match query!(
            "DELETE FROM user_keys WHERE key_id = $1", 
            key_id
        )
            .execute(pool)
            .await 
        {
            Ok(_feedbck) => {},
            Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
        };
        Ok(del_op)
    }
    else {
        let e: String = format!("User is not an administrator.");
        Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    } 
}

/// this function attempts to
/// change the status of a user
/// key to being valid. If the operation
/// is successful, an empty function is returned.
/// If this operation fails, an error is returned.
pub async fn change_user_key_status(
    key_id: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr> { 
    let update_op: () = match query!(
        "UPDATE user_keys SET key_used = $1 WHERE key_id = $2", 
        true,
        key_id
    )
        .execute(pool)
        .await 
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(update_op)
}

/// This function attempts to retrieve
/// all keys an administrator has created.
/// If the operation is successful, a vector
/// of instances of the "UserKey" structure
/// is returned. If this operation fails,
/// an error is returned.
pub async fn get_user_keys(
    api_token: &String,
    pool: &Pool<Postgres>,
) -> Result<Vec<UserKey>, CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<Vec<UserKey>, CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.is_admin{
        let user_keys: Vec<UserKey> = match query_as!(
            UserKey,
            "SELECT * FROM user_keys WHERE user_id = $1",
            user.user_id
        )   
            .fetch_all(pool)
            .await 
        {
            Ok(user_keys) => user_keys,
            Err(e) => return Err::<Vec<UserKey>, CleoErr>(CleoErr::new(&e.to_string()))
        };
        Ok(user_keys)
    }
    else {
        let e: String = format!("User is not an administrator.");
        Err::<Vec<UserKey>, CleoErr>(CleoErr::new(&e.to_string()))
    }
}

/// This function attempts
/// to check whether a user
/// key exists or not. Depending
/// on this a boolean is returned.
/// If the operation fails, an error
/// is returned.
pub async fn user_key_exists(
    user_key: &String,
    pool: &Pool<Postgres>,
) -> bool {
    let exists: bool = match query_as!(
        UserKey,
        "SELECT * FROM user_keys WHERE user_key = $1",
        user_key
    )   
        .fetch_one(pool)
        .await 
    {
        Ok(_fetched_key) => true,
        Err(e) => false
    };
    exists
}
