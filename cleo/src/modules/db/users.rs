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
    let key_valid: bool = match user_key_exists(user_key, pool).await {
        Ok(key_valid) => key_valid,
        Err(e) => return Err::<CleoUser, CleoErr>(CleoErr::new(&e.to_string()))
    };
    if key_valid && key_length == 16 {
        is_admin = true;
    }
    else if key_valid && key_length == 10 {
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
        email_verif_code: hash_string(
            &format!(
                "{}{}", 
                TimeNow::new().to_string(), 
                &username
            )
        ),
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

// Done.
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

// Done.
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
    let user: CleoUser = match get_user_by_id(user_id, pool).await {
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

pub async fn get_user_from_email_token(
    token: &str,
    pool: &Pool<Postgres>
) -> Result<CleoUser, CleoErr> {
    let uat_obj: CleoUser = match query_as!(
        UserAPIToken,
        "SELECT * FROM cleo_users WHERE email_token_verif = $1", 
        token
    )
        .fetch_one(pool)
        .await 
    {
        Ok(uat_obj) => uat_obj,
        Err(e) => return Err::<CleoUser, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(uat_obj)
}

// Done. Has service.
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

// Done. Has service.
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

// Done.
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

// Done. Has service.
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
        Ok(_feedbck) => {},
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(update_op)
}

// done. Has service.
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

// Done. Has service.
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

// Done. Has service.
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

// Done.
pub async fn user_exists(
    user_id: &String,
    pool: &Pool<Postgres>,
) -> Result<bool, CleoErr> {
    let fetched_user: CleoUser = match query_as!(
        CleoUser,
        "SELECT * FROM cleo_users WHERE user_id = $1",
        user_id
    )   
        .fetch_one(pool)
        .await 
    {
        Ok(fetched_user) => fetched_user,
        Err(e) => return Err::<bool, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let mut result: bool = false;
    if &fetched_user.user_id == user_id {
        result = true;
    }
    else {}
    Ok(result)
}
