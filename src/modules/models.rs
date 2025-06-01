/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// "FromRow" derive
/// macro to
/// create and read
/// tables from the
/// database.
use sqlx::FromRow;

/// A structure
/// for creating tables
/// for holding info
/// on the Cleo instance.
#[derive(FromRow)]
pub struct InstanceInformation{
    pub instance_id: String,
    pub hostname: String,
    pub instance_name: String,
    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_pass: String,
    pub file_dir: String
}

/// A structure
/// for creating tables
/// for holding info
/// on Cleo users.
#[derive(FromRow)]
pub struct CleoUser{
    pub user_id: String,
    pub display_name: String,
    pub is_verified: bool,
    pub username: String,
    pub pwd: String,
    pub email_addr: String,
    pub pfp_url: String,
    pub is_admin: bool,
}

/// A structure
/// for creating tables
/// for holding info
/// on files Cleo
/// users have uploaded.
#[derive(FromRow)]
pub struct UserFile{
    pub file_id: String,
    pub user_id: String,
    pub file_path: String,
    pub file_url: String
}

/// A structure
/// for creating tables
/// for holding info
/// on posts Cleo
/// users created.
#[derive(FromRow)]
pub struct UserPost{
    pub content_id: String,
    pub user_id: String,
    pub content_type: String,
    pub content_text: String,
}

/// A structure
/// for creating tables
/// for holding info
/// on extra key-value 
/// info a Cleo user
/// set for a post.
#[derive(FromRow)]
pub struct ExtraContentField{
    pub field_id: String,
    pub content_id: String,
    pub field_key: String,
    pub field_value: String
}

/// A structure
/// for creating tables
/// for holding info
/// on API tokens belonging
/// to a certain Cleo user.
#[derive(FromRow)]
pub struct UserAPIToken {
    pub token_id: String,
    pub user_id: String,
    pub token: String
}

/// A structure
/// for creating tables
/// for holding info
/// on keys issued by
/// the administrator
/// for enabling
/// others to create
/// an account on
/// this instance.
#[derive(FromRow)]
pub struct UserKey {
    pub key_id: String,
    pub user_id: String,
    pub user_key: String,
    pub key_type: String,
    pub key_used: bool,
    pub username: String
}

/// A structure
/// for creating tables
/// for holding info
/// on email tokens.
#[derive(FromRow)]
pub struct EmailToken{
  pub etoken_id: String,
  pub email_token: String,
  pub user_id: String
}
