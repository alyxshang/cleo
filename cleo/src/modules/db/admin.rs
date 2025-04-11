/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

use sqlx::Pool;
use sqlx::query;
use sqlx::query_as;
use sqlx::postgres::Postgres;
use crate::modules::err::CleoErr;
use crate::modules::models::UserKey;
use crate::modules::models::CleoUser;
use crate::modules::utils::hash_string;
use super::tokens::get_user_from_token;
use crate::modules::models::InstanceInformation;

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
