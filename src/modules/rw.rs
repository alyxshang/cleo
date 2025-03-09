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

// Done. Has service.
pub async fn create_user_post(
    api_token: &String,
    content_type: &String, // "page" or "post"
    content_text: &String, // needs to be html
    pool: &Pool<Postgres>
) -> Result<UserPost, CleoErr>{
    let user_obj: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user_obj) => user_obj,
        Err(e) => return Err::<UserPost, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let content_snippet: String = content_text.chars().take(16).collect();
    let content_id: String = hash_string(
        &format!(
            "{}{}",
            TimeNow::new().to_string(),
            content_snippet
        )
    );
    let post_obj: UserPost = UserPost{
        user_id: user_obj.user_id,
        content_text: content_text.clone(),
        content_type: content_type.to_owned(),
        content_id: content_id.clone(),
    };
    let _insert_op = match query!(
        "INSERT INTO user_posts (user_id, content_text, content_type, content_id) VALUES ($1, $2, $3, $4)",
        post_obj.user_id,
        post_obj.content_text,
        post_obj.content_type,
        post_obj.content_id
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<UserPost, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let post_obj: UserPost = match get_post_by_id(&content_id, pool).await {
        Ok(post_obj) => post_obj,
        Err(e) => return Err::<UserPost, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(post_obj)
}

// Done.
pub async fn get_post_by_id(
    content_id: &String,
    pool: &Pool<Postgres>
) -> Result<UserPost, CleoErr> {
    let post_obj: UserPost = match query_as!(
        UserPost,
        "SELECT * FROM user_posts WHERE content_id = $1", 
        content_id
    )
        .fetch_one(pool)
        .await 
    {
        Ok(post_obj) => post_obj,
        Err(e) => return Err::<UserPost, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(post_obj)
}

// Done. Has service.
pub async fn update_post_text(
    api_token: &String,
    content_id: &String,
    text: &String,
    pool: &Pool<Postgres>
) -> Result<(), CleoErr> {
    let user_obj: CleoUser = match get_user_from_token(&api_token, pool).await {
        Ok(user_obj) => user_obj,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let post_obj: UserPost = match get_post_by_id(&content_id, pool).await {
        Ok(post_obj) => post_obj,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user_obj.user_id == post_obj.user_id {
        let update_op: () = match query!(
            "UPDATE user_posts SET content_text = $1 WHERE content_id = $2", 
            text,
            content_id
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
        let e: String = "Could not verify ownership of the post.".to_string();
        Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    }
}

// Done. Has service.
pub async fn delete_post(
    api_token: &String,
    content_id: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr> {
    let user_obj: CleoUser = match get_user_from_token(&api_token, pool).await {
        Ok(user_obj) => user_obj,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let post_obj: UserPost = match get_post_by_id(&content_id, pool).await {
        Ok(post_obj) => post_obj,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user_obj.user_id == post_obj.user_id {
        let del_op: () = match query!(
            "DELETE FROM user_posts WHERE content_id = $1", 
            content_id
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
        let e: String = "Could not verify ownership of the post.".to_string();
        Err::<(), CleoErr>(CleoErr::new(&e.to_string()))

    }
}

// Done. Has service.
pub async fn create_extra_field_for_post(
    api_token: &String,
    content_id: &String,
    field_key: &String,
    field_value: &String,
    pool: &Pool<Postgres>,
) -> Result<ExtraContentField, CleoErr> {    
    let post: UserPost = match get_post_by_id(content_id, pool).await {
        Ok(post) => post,
        Err(e) => return Err::<ExtraContentField, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<ExtraContentField, CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.user_id == post.user_id {
        let field_id: String = format!("{}{}", &field_value, TimeNow::new().to_string());
        let extra_field: ExtraContentField = ExtraContentField{
            field_id: field_id.clone(),
            content_id: content_id.to_owned(),
            field_key: field_key.to_owned(),
            field_value: field_value.to_owned()
        };
        let _insert_op = match query!(
            "INSERT INTO extra_content_fields (field_id, content_id, field_key, field_value) VALUES ($1, $2, $3, $4)",
            extra_field.field_id,
            extra_field.content_id,
            extra_field.field_key,
            extra_field.field_value
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<ExtraContentField, CleoErr>(CleoErr::new(&e.to_string()))
        };
        let field_obj: ExtraContentField = match get_extra_field_by_id(&field_id, pool).await {
            Ok(field_obj) => field_obj,
            Err(e) => return Err::<ExtraContentField, CleoErr>(CleoErr::new(&e.to_string()))
        };
        Ok(field_obj)
    }
    else {
        let e: String = format!("Could not verify ownership of token.");
        Err::<ExtraContentField, CleoErr>(CleoErr::new(&e.to_string()))
    }
}

// Done.
pub async fn get_extra_field_by_id(
    field_id: &String,
    pool: &Pool<Postgres>
) -> Result<ExtraContentField, CleoErr> {
    let field_obj: ExtraContentField = match query_as!(
        ExtraContentField,
        "SELECT * FROM extra_content_fields WHERE field_id = $1", 
        field_id
    )
        .fetch_one(pool)
        .await 
    {
        Ok(field_obj) => field_obj,
        Err(e) => return Err::<ExtraContentField, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(field_obj)
}

// Done. Has service.
pub async fn delete_extra_field_for_post(
    api_token: &String,
    content_id: &String,
    pool: &Pool<Postgres>,
    field_id: &String,
) -> Result<(), CleoErr> {  
    let post: UserPost = match get_post_by_id(content_id, pool).await {
        Ok(post) => post,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.user_id == post.user_id {
        let del_op: () = match query!(
            "DELETE FROM extra_content_fields WHERE field_id = $1", 
            field_id
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
        let e: String = format!("Could not verify ownership of token.");
        Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    }  
}

// Done. Has service.
pub async fn edit_extra_field_key_for_post(
    api_token: &String,
    content_id: &String,
    field_id: &String,
    field_key_new: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr> {    
    let post: UserPost = match get_post_by_id(content_id, pool).await {
        Ok(post) => post,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.user_id == post.user_id {
        let update_op: () = match query!(
            "UPDATE extra_content_fields SET field_key = $1 WHERE field_id = $2", 
            field_key_new,
            field_id
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
        let e: String = format!("Could not verify ownership of token.");
        Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    } 
}

// Done. Has service.
pub async fn edit_extra_field_value_for_post(
    api_token: &String,
    content_id: &String,
    field_id: &String,
    field_value_new: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr> {    
    let post: UserPost = match get_post_by_id(content_id, pool).await {
        Ok(post) => post,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.user_id == post.user_id {
        let update_op: () = match query!(
            "UPDATE extra_content_fields SET field_value = $1 WHERE field_id = $2", 
            field_value_new,
            field_id
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
        let e: String = format!("Could not verify ownership of token.");
        Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    } 
}

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
pub async fn get_instance_users(
    api_token: &String,
    pool: &Pool<Postgres>,
) -> Result<Vec<CleoUser>, CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<Vec<CleoUser>, CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.is_admin {
        let cleo_users: Vec<CleoUser> = match query_as!(
            CleoUser,
            "SELECT * FROM cleo_users WHERE is_admin = $1",
            false
        )   
            .fetch_all(pool)
            .await 
        {
            Ok(cleo_users) => cleo_users,
            Err(e) => return Err::<Vec<CleoUser>, CleoErr>(CleoErr::new(&e.to_string()))
        };
        Ok(cleo_users)
    }
    else {
        let e: String = format!("User is not an administrator.");
        Err::<Vec<CleoUser>, CleoErr>(CleoErr::new(&e.to_string()))
    }
}

// Done. Has service.
pub async fn get_instance_admins(
    api_token: &String,
    pool: &Pool<Postgres>,
) -> Result<Vec<CleoUser>, CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<Vec<CleoUser>, CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.is_admin {
        let cleo_users: Vec<CleoUser> = match query_as!(
            CleoUser,
            "SELECT * FROM cleo_users WHERE is_admin = $1",
            true
        )   
            .fetch_all(pool)
            .await 
        {
            Ok(cleo_users) => cleo_users,
            Err(e) => return Err::<Vec<CleoUser>, CleoErr>(CleoErr::new(&e.to_string()))
        };
        Ok(cleo_users)
    }
    else {
        let e: String = format!("User is not an administrator.");
        Err::<Vec<CleoUser>, CleoErr>(CleoErr::new(&e.to_string()))
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

// Done.
pub async fn create_instance_info(
    pool: &Pool<Postgres>,
    smtp_server: &String,
    hostname: &String,
    instance_name: &String,
    smtp_username: &String,
    smtp_pass: &String
) -> Result<usize, CleoErr> {
    let hashed_source: String = format!("{}{}", &hostname, &instance_name);
    let instance_id: String = hash_string(&hashed_source);
    let mut result: usize = 1;
    let info_obj: InstanceInformation = InstanceInformation{
        instance_id: instance_id,
        hostname: hostname.to_owned(),
        instance_name: instance_name.to_owned(),
        smtp_server: smtp_server.to_owned(),
        smtp_username: smtp_username.to_owned(),
        smtp_pass: smtp_pass.to_owned()
    };
    let _insert_op = match query!(
        "INSERT INTO instance_info (instance_id, hostname, instance_name, smtp_server, smtp_username, smtp_pass) VALUES ($1, $2, $3, $4, $5, $6)",
        info_obj.instance_id,
        info_obj.hostname,
        info_obj.instance_name,
        info_obj.smtp_server,
        info_obj.smtp_username,
        info_obj.smtp_pass
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => { result = 0 },
        Err(e) => return Err::<usize, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(result)
}

// Done. // Has service.
pub async fn edit_instance_hostname(
    api_token: &String,
    new_hostname: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let info: InstanceInformation = match get_instance_info(pool).await {
        Ok(info) => info,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.is_admin{
        let update_op: () = match query!(
            "UPDATE instance_info SET hostname = $1 WHERE instance_id = $2",
            new_hostname, 
            info.instance_id
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
        let e: &str = "The acting user must be an administrator.";
        return Err::<(), CleoErr>(CleoErr::new(&e.to_string()));
    }
}

// Done. // Has service.
pub async fn edit_instance_name(
    api_token: &String,
    new_name: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let info: InstanceInformation = match get_instance_info(pool).await {
        Ok(info) => info,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.is_admin{
        let update_op: () = match query!(
            "UPDATE instance_info SET instance_name = $1 WHERE instance_id = $2",
            new_name, 
            info.instance_id
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
        let e: &str = "The acting user must be an administrator.";
        return Err::<(), CleoErr>(CleoErr::new(&e.to_string()));
    }
}

// Done. // Has service.
pub async fn edit_smtp_username(
    api_token: &String,
    new_name: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let info: InstanceInformation = match get_instance_info(pool).await {
        Ok(info) => info,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.is_admin{
        let update_op: () = match query!(
            "UPDATE instance_info SET smtp_username = $1 WHERE instance_id = $2",
            new_name, 
            info.instance_id
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
        let e: &str = "The acting user must be an administrator.";
        return Err::<(), CleoErr>(CleoErr::new(&e.to_string()));
    }
}

// Done. // Has service.
pub async fn edit_smtp_pass(
    api_token: &String,
    new_name: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let info: InstanceInformation = match get_instance_info(pool).await {
        Ok(info) => info,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.is_admin{
        let update_op: () = match query!(
            "UPDATE instance_info SET smtp_pass = $1 WHERE instance_id = $2",
            new_name, 
            info.instance_id
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
        let e: &str = "The acting user must be an administrator.";
        return Err::<(), CleoErr>(CleoErr::new(&e.to_string()));
    }
}

// Done. Has service.
pub async fn edit_instance_smtp_server(
    api_token: &String,
    new_smtp_server: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let info: InstanceInformation = match get_instance_info(pool).await {
        Ok(info) => info,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.is_admin{
        let update_op: () = match query!(
            "UPDATE instance_info SET smtp_server = $1 WHERE instance_id = $2",
            new_smtp_server, 
            info.instance_id
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
        let e: &str = "The acting user must be an administrator.";
        return Err::<(), CleoErr>(CleoErr::new(&e.to_string()));
    }
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