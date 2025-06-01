/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the 
/// "Pool" structure
/// to accept multiple
/// connections to the 
/// database.
use sqlx::Pool;

/// Importing the "query"
/// macro to execute SQL
/// queries to return
/// nothing.
use sqlx::query;

/// Importing the
/// function to verify
/// a hashed string.
use bcrypt::verify;

/// Importing the "query_as"
/// macro to execute SQL
/// queries to return
/// something.
use sqlx::query_as;

/// Importing the "Postgres"
/// structure to specify which
/// database one is connecting
/// to.
use sqlx::postgres::Postgres;

/// Importing the "CleoErr"
/// structure to catch and handle
/// errors.
use crate::modules::err::CleoErr;

/// Importing the "TimeNow" structure
/// to get the current time.
use crate::modules::utils::TimeNow;

/// Importing the "CleoUser" structure
/// to read and write information about
/// a Cleo user.
use crate::modules::models::CleoUser;

/// Importing the function to hash 
/// a string.
use crate::modules::utils::hash_string;

/// Importing the structure for the model
/// for a user's API tokens to read and write
/// from and to the database about these
/// entities.
use crate::modules::models::UserAPIToken;

/// Importing the "get_user_from_token"
/// function to retrieve a Cleo
/// user using their ID.
use crate::modules::db::users::get_user_by_id;

/// Importing the "get_user_from_token"
/// function to retrieve a Cleo
/// user using their username.
use crate::modules::db::users::get_user_by_username;

/// This function attempts to create
/// an API token for a user. If this operation
/// is successful, an instance of the "UserAPIToken"
/// structure is returned. If this operation fails,
/// an error is returned.
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

/// This function attempts to
/// retrieve an instance of the
/// "UserAPIToken" structure given
/// the token. If this operation fails,
/// an error is returned.
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

/// This function attempts to
/// delete an API token for a user.
/// If this operation is successful,
/// an empty function is returned.
/// If this operation fails, an error
/// is returned.
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

/// This function attempts
/// to retrieve an instance of
/// the "CleoUser" structure given
/// an API token. If this operation
/// fails, an error is returned.
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
