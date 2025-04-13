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

/// Importing the function
/// to hash a string using
/// the "BCrypt" algorithm.
use bcrypt::hash;

/// Importing the function
/// to verify a hashed string.
use bcrypt::verify;

/// Importing the "query_as"
/// macro to execute SQL
/// queries to return
/// something.
use sqlx::query_as;

/// Importing the "DEFAULT_COST"
/// enum from the "bcrypt" token
/// to specify the computational
/// cost for hashing a string.
use bcrypt::DEFAULT_COST;

/// Importing the "Postgres"
/// structure for explicit 
/// typing.
use sqlx::postgres::Postgres;

/// Importing the "CleoErr"
/// structure to catch and
/// handle errors.
use crate::modules::err::CleoErr;

/// Importing the "TimeNow"
/// structure to get the current
/// time.
use crate::modules::utils::TimeNow;

/// Importing the structure that
/// models user keys in the 
/// database.
use crate::modules::models::UserKey;

/// Importing the "CleoUser" structure
/// for explicit typing.
use crate::modules::models::CleoUser;

/// Importing the function to generate
/// a general hash of a string.
use crate::modules::utils::hash_string;

/// Importing the structure for the model
/// for a user's API tokens to read and write
/// from and to the database about these
/// entities.
use crate::modules::models::UserAPIToken;

/// Importing the function to get an entry
/// in the database for an API token given
/// the token itself.
use crate::modules::db::tokens::get_token;

/// Importing the function to check whether
/// a user key exists. This is relevant for
/// signing users up.
use crate::modules::db::keys::user_key_exists;

/// Importing the function to retrieve a user
/// key from the database given the key.
use crate::modules::db::keys::get_user_key_by_id;

/// This function attempts
/// to create a user in the database.
/// If this operation fails, an error 
/// is returned. If this operation is
/// successful, an instance of the
/// "CleoUser" structure is
/// returned.
pub async fn create_user(
    username: &String,
    display_name: &String,
    password: &String,
    email_addr: &String,
    pfp_url: &String,
    user_key: &String,
    pool: &Pool<Postgres>
) -> Result<CleoUser, CleoErr>{
    let hashed_source: String = format!(
        "{}{}", 
        TimeNow::new().to_string(),
        username
    );
    let hashed_pwd: String = match hash(password, DEFAULT_COST){
        Ok(hashed_pwd) => hashed_pwd,
        Err(e) => return Err::<CleoUser, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let user_id: String = hash_string(&hashed_source);
    let key_length: usize = user_key.chars().collect::<Vec<char>>().len();
    let is_admin: bool;
    let key_valid: bool = user_key_exists(user_key, pool).await;
    let user_exists: bool = user_exists_by_username(username, pool).await;
    let user_key_obj: UserKey = match get_user_key_by_id(user_key, pool).await {
        Ok(user_key) => user_key,
        Err(e) => return Err::<CleoUser, CleoErr>(CleoErr::new(&e.to_string()))
    };
    if key_valid && key_length == 16 && username == &user_key_obj.username && user_exists == false {
        is_admin = true;    
    }
    else if key_valid && key_length == 10 && username == &user_key_obj.username && user_exists == false {
        is_admin = false;
    }
    else {
        let e: &str = "Could not create account with the provided information.";
        return Err::<CleoUser, CleoErr>(CleoErr::new(&e.to_string()));
    }
    let user_obj: CleoUser = CleoUser {
        user_id: user_id.clone(), 
        display_name: display_name.to_owned(),
        is_verified: false,
        username: username.clone(),
        pwd: hashed_pwd,  
        email_addr: email_addr.to_owned(),
        pfp_url: pfp_url.to_owned(), 
        is_admin: is_admin.to_owned()
    };
    let _insert_op = match query!(
        "INSERT INTO cleo_users (user_id, display_name, is_verified, username, pwd, email_addr, pfp_url, is_admin) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        user_obj.user_id,
        user_obj.display_name,
        user_obj.is_verified,
        user_obj.username,
        user_obj.pwd,
        user_obj.email_addr,
        user_obj.pfp_url,
        user_obj.is_admin,
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<CleoUser, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let res: CleoUser = match get_user_by_id(&user_id, pool).await {
        Ok(res) => res,
        Err(e) => return Err::<CleoUser, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(res)
}

/// This function attempts to retrieve
/// an instance of the "CleoUser" structure
/// given a user's ID. If this operation
/// fails, an error is returned.
pub async fn get_user_by_id(
    id: &String, 
    pool: &Pool<Postgres>
) -> Result<CleoUser, CleoErr> {
    let user_obj: CleoUser = match query_as!(CleoUser,"SELECT * FROM cleo_users WHERE user_id = $1", id).fetch_one(pool).await {
        Ok(user_obj) => user_obj,
        Err(e) => return Err::<CleoUser, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(user_obj)
}

/// This function attempts to retrieve
/// an instance of the "CleoUser" structure
/// given a user's handle. If this operation
/// fails, an error is returned.
pub async fn get_user_by_username(
    username: &String, 
    pool: &Pool<Postgres>
) -> Result<CleoUser, CleoErr> {
    let user_obj: CleoUser = match query_as!(CleoUser,"SELECT * FROM cleo_users WHERE username = $1", username).fetch_one(pool).await {
        Ok(user_obj) => user_obj,
        Err(e) => return Err::<CleoUser, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(user_obj)
}

pub async fn set_verified(
    user_id: &str,
    pool: &Pool<Postgres>
) -> Result<(), CleoErr> {
    let user: CleoUser = match get_user_by_id(&user_id.to_string(), &pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let update_op: () = match query!(
        "UPDATE cleo_users SET is_verified = $1 WHERE user_id = $2",
        true, 
        user.user_id
    )
        .execute(pool)
        .await 
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(update_op)
}

/// This function attempts to
/// update the username of a user.
/// If this operation is successful,
/// an empty function is returned.
/// If this operation fails, an error
/// is returned.
pub async fn update_username(
    api_token: &String,
    new_username: &String,
    pool: &Pool<Postgres>
) -> Result<(), CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let update_op: () = match query!(
        "UPDATE cleo_users SET username = $1 WHERE user_id = $2",
        new_username, 
        user.user_id
    )
        .execute(pool)
        .await 
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(update_op)
}

/// This function attempts to
/// update the email of a user.
/// If this operation is successful,
/// an empty function is returned.
/// If this operation fails, an error
/// is returned.
pub async fn update_email(
    api_token: &String,
    new_email: &String,
    pool: &Pool<Postgres>
) -> Result<(), CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let update_op: () = match query!(
        "UPDATE cleo_users SET email_addr = $1 WHERE user_id = $2",
        new_email, 
        user.user_id
    )
        .execute(pool)
        .await 
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(update_op)
}
/// This function attempts to
/// update the name of a user.
/// If this operation is successful,
/// an empty function is returned.
/// If this operation fails, an error
/// is returned.
pub async fn update_display_name(
    api_token: &String,
    new_name: &String,
    pool: &Pool<Postgres>
) -> Result<(), CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let update_op: () = match query!(
        "UPDATE cleo_users SET display_name = $1 WHERE user_id = $2",
        new_name, 
        user.user_id
    )
        .execute(pool)
        .await 
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(update_op)
}

/// This function attempts to
/// update the verified-status of a user.
/// If this operation is successful,
/// an empty function is returned.
/// If this operation fails, an error
/// is returned.
pub async fn update_verified(
    verified: &bool,
    user_id: &String,
    pool: &Pool<Postgres>
) -> Result<(), CleoErr>{
    let update_op: () = match query!(
        "UPDATE cleo_users SET is_verified = $1 WHERE user_id = $2",
        verified, 
        user_id
    )
        .execute(pool)
        .await 
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(update_op)
}

/// This function attempts to
/// update the profile picture of a user.
/// If this operation is successful,
/// an empty function is returned.
/// If this operation fails, an error
/// is returned.
pub async fn update_pfp(
    api_token: &String,
    new_pfp_url: &String,
    pool: &Pool<Postgres>
) -> Result<(), CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let update_op: () = match query!(
        "UPDATE cleo_users SET pfp_url = $1 WHERE user_id = $2",
        new_pfp_url, 
        user.user_id
    )
        .execute(pool)
        .await 
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(update_op)
}

/// This function attempts to
/// update the password of a user.
/// If this operation is successful,
/// an empty function is returned.
/// If this operation fails, an error
/// is returned.
pub async fn update_password(
    api_token: &String,
    new_password: &String,
    pool: &Pool<Postgres>
) -> Result<(), CleoErr>{
    let user_obj: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user_obj) => user_obj,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let hashed_pwd: String = match hash(new_password, DEFAULT_COST){
        Ok(hashed_pwd) => hashed_pwd,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let update_op: () = match query!(
        "UPDATE cleo_users SET pwd = $1 WHERE user_id = $2", 
        hashed_pwd,
        user_obj.user_id
    )
        .execute(pool)
        .await 
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(update_op)
}

/// This function attempts to delete
/// a user created in the database.
/// If this operation is successful,
/// an empty function is returned.
/// If this operation fails, an error
/// is returned.
pub async fn delete_user_from_db(
    username: &String,
    password: &String,
    pool: &Pool<Postgres>
) -> Result<(), CleoErr>{
    let user_obj: CleoUser = match get_user_by_username(username, pool).await {
        Ok(user_obj) => user_obj,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let verified: bool = match verify(password, &user_obj.pwd){
        Ok(verified) => verified,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if verified{
        let del_op: () = match query!(
            "DELETE FROM cleo_users WHERE user_id = $1", 
            user_obj.user_id
        )
            .execute(pool)
            .await 
        {
            Ok(_user_obj) => {},
            Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
        };
        Ok(del_op)
    }
    else {
        let e: String = format!("Could not verify password for user with the username \"{}\"", username);
        Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    }
}

/// This function attempts to retrieve
/// an instance of the "CleoUser" sructure
/// from the database given their API token.
/// If this operation fails, an error is returned.
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

/// This function attempts
/// to check wheter a user exists given
/// the user's ID in the database. 
/// Depending on this, a boolean is 
/// returned. If this operation
/// fails, an error is returned.
pub async fn user_exists_by_user_id(
    user_id: &String,
    pool: &Pool<Postgres>,
) -> bool {
    let fetched_user: bool = match query_as!(
        CleoUser,
        "SELECT * FROM cleo_users WHERE user_id = $1",
        user_id
    )   
        .fetch_one(pool)
        .await 
    {
        Ok(fetched_user) => true,
        Err(e) => false
    };
    fetched_user
}

/// This function attempts
/// to check wheter a user exists given
/// the user's handle in the database. 
/// Depending on this, a boolean is 
/// returned. If this operation
/// fails, an error is returned.
pub async fn user_exists_by_username(
    username: &String,
    pool: &Pool<Postgres>,
) -> bool {
    let fetched_user: bool = match query_as!(
        CleoUser,
        "SELECT * FROM cleo_users WHERE username = $1",
        username
    )   
        .fetch_one(pool)
        .await 
    {
        Ok(fetched_user) => true,
        Err(e) => false
    };
    fetched_user
}
