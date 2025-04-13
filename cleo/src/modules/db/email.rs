/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Pool"
/// structure to use a pool
/// of connections.
use sqlx::Pool;

/// Importing the "query"
/// macro to execute queries
/// that return nothing.
use sqlx::query;

/// Importing the "query_as"
/// macro to execute queries
/// that return something.
use sqlx::query_as;

/// Importing the "Postgres"
/// structure for explicit 
/// typing.
use sqlx::postgres::Postgres;

/// Importing the "CleoErr"
/// structure to catch and
/// handle errors.
use crate::modules::err::CleoErr;

/// Importing the "TimeNow" structure
/// to get the current time.
use crate::modules::utils::TimeNow;

/// Importing the function to hash 
/// a string.
use crate::modules::utils::hash_string;

//// Importing the structure
/// for modelling email tokens
/// in the database.
use crate::modules::models::EmailToken;

/// This function attempts to
/// create an email token for
/// a user in the database. If the
/// operation is successful, the created
/// object is returned as an instance of
/// the "EmailToken" structure. If the
/// operation fails, an error is returned..
pub async fn create_email_token(
    user_id: &String,
    pool: &Pool<Postgres>
) -> Result<EmailToken, CleoErr>{
    let email_token: String = hash_string(
        &format!(
            "{}:{}",
            user_id,
            TimeNow::new().to_string()
        )
    );
    let token_id: String = hash_string(&TimeNow::new().to_string());
    let token: EmailToken = EmailToken{
        etoken_id: token_id,
        email_token: email_token.clone(),
        user_id: user_id.to_string()
    };
    let _insert_op = match query!(
            "INSERT INTO email_tokens (etoken_id, email_token, user_id) VALUES ($1, $2, $3)",
            token.etoken_id,
            token.email_token,
            token.user_id
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<EmailToken, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let token_obj: EmailToken = match get_object_from_token(&email_token, pool).await {
        Ok(field_obj) => field_obj,
        Err(e) => return Err::<EmailToken, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(token_obj)
}

/// This function attempts
/// to retrieve an object of
/// an email token given a 
/// token. If the operation is
/// successful, an instance of
/// the "EmailToken" structure
/// is returned. If the operation
/// fails, an error is returned.
pub async fn get_object_from_token(
    token: &String,
    pool: &Pool<Postgres>
) -> Result<EmailToken, CleoErr>{
    let token_obj: EmailToken = match query_as!(
        EmailToken,
        "SELECT * FROM email_tokens WHERE email_token = $1", 
        token
    )
        .fetch_one(pool)
        .await 
    {
        Ok(token_obj) => token_obj,
        Err(e) => return Err::<EmailToken, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(token_obj)
}

/// This function attempts to delete
/// an email token given its
/// ID. If the operation is successful,
/// an empty function is returned. If the 
/// operation fails, an error is returned.
pub async fn delete_email_token(
    token: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr> {  
    let del_op: () = match query!(
        "DELETE FROM email_tokens WHERE email_token = $1", 
        token
    )
        .execute(pool)
        .await 
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(del_op)  
}
