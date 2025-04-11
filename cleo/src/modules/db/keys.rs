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
pub async fn create_user_key(
    key_type: &String,
    api_token: &String,
    pool: &Pool<Postgres>,
) -> Result<UserKey, CleoErr> {
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<UserKey, CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.is_admin{
        let user_key: String = match generate_key(&16){
            Ok(user_key) => user_key,
            Err(e) => return Err::<UserKey, CleoErr>(CleoErr::new(&e.to_string()))
        };
        let key_id: String = format!("{}{}", &user_key, TimeNow::new().to_string());
        let user_key_obj: UserKey = UserKey{
            key_id: key_id.clone(),
            user_id: user.user_id,
            user_key: user_key,
            key_type: key_type.to_owned(),
            key_used: false
        };
        let _insert_op = match query!(
            "INSERT INTO user_keys (key_id, user_id, user_key, key_type, key_used) VALUES ($1, $2, $3, $4, $5)",
            user_key_obj.key_id,
            user_key_obj.user_id,
            user_key_obj.user_key,
            user_key_obj.key_type,
            user_key_obj.key_used
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

// Done.
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

// Done. Has service.
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

// Done.
pub async fn change_user_key_status(
    api_token: &String,
    key_id: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr> {
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.is_admin {
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
    else {
        let e: String = format!("User is not an administrator.");
        Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    }
}

// Done. Has service.
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
// Done.
pub async fn user_key_exists(
    user_key: &String,
    pool: &Pool<Postgres>,
) -> Result<bool, CleoErr> {
    let fetched_key: UserKey = match query_as!(
        UserKey,
        "SELECT * FROM user_keys WHERE user_key = $1",
        user_key
    )   
        .fetch_one(pool)
        .await 
    {
        Ok(fetched_key) => fetched_key,
        Err(e) => return Err::<bool, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let mut result: bool = false;
    if &fetched_key.user_key == user_key {
        result = true;
    }
    else {}
    Ok(result)
}
