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
pub async fn create_api_token_for_user(
    username: &String,
    password: &String,
    pool: &Pool<Postgres>
) -> Result<UserAPIToken, CleoErr> {
    let user_obj: CleoUser = match get_user_by_username(username, pool).await {
        Ok(user_obj) => user_obj,
        Err(e) => return Err::<UserAPIToken, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let verified: bool = match verify(password, &user_obj.pwd){
        Ok(verified) => verified,
        Err(e) => return Err::<UserAPIToken, CleoErr>(CleoErr::new(&e.to_string()))
    };
    if verified{
        let token_id: String = hash_string(&format!("{}{}", user_obj.username, TimeNow::new().to_string()));
        let token: String = hash_string(&format!("{}{}", user_obj.user_id, TimeNow::new().to_string()));
        let uat_obj: UserAPIToken = UserAPIToken{
            user_id: user_obj.user_id.clone(),
            token_id: token_id,
            token: token.clone()
        };
        let _insert_op = match query!(
            "INSERT INTO user_api_tokens (user_id, token_id, token) VALUES ($1, $2, $3)",
            uat_obj.user_id,
            uat_obj.token_id,
            uat_obj.token
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<UserAPIToken, CleoErr>(CleoErr::new(&e.to_string()))
        };
        let token_obj: UserAPIToken = match get_token(&token, pool).await {
            Ok(token_obj) => token_obj,
            Err(e) => return Err::<UserAPIToken, CleoErr>(CleoErr::new(&e.to_string()))
        };
        Ok(token_obj)
    }
    else {
        let e: String = format!("Could not verify password for user with the username \"{}\"", username);
        Err::<UserAPIToken, CleoErr>(CleoErr::new(&e.to_string()))
    }
}

// Done.
pub async fn get_token(
    token: &String,  
    pool: &Pool<Postgres>
) -> Result<UserAPIToken, CleoErr>{
    let uat_obj: UserAPIToken = match query_as!(
        UserAPIToken,
        "SELECT * FROM user_api_tokens WHERE token = $1", 
        token
    )
        .fetch_one(pool)
        .await 
    {
        Ok(uat_obj) => uat_obj,
        Err(e) => return Err::<UserAPIToken, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(uat_obj)
}

// Done. Has service.
pub async fn delete_token(
    token: &String,
    username: &String,
    password: &String,
    pool: &Pool<Postgres>
) -> Result<(), CleoErr> {
    let user_obj: CleoUser = match get_user_by_username(username, pool).await {
        Ok(user_obj) => user_obj,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let verified: bool = match verify(password, &user_obj.pwd){
        Ok(verified) => verified,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if verified{
        let token_obj: UserAPIToken = match get_token(token, pool).await {
            Ok(token_obj) => token_obj,
            Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
        };
        let del_op: () = match query!(
            "DELETE FROM user_api_tokens WHERE token_id = $1", 
            token_obj.token_id
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
        let e: String = format!("Could not verify password for user with the username \"{}\"", username);
        Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    }
}

// Done.
pub async fn get_user_from_token(
    token: &String,
    pool: &Pool<Postgres>
) -> Result<CleoUser, CleoErr>{
    let token_obj: UserAPIToken = match get_token(token, pool).await {
        Ok(token_obj) => token_obj,
        Err(e) => return Err::<CleoUser, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let user_obj: CleoUser = match get_user_by_id(&token_obj.user_id, pool).await {
        Ok(user_obj) => user_obj,
        Err(e) => return Err::<CleoUser, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(user_obj)
}
