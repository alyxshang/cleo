/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// "Pool" structure
/// from the "sqlx" crate
/// to make a pool for
/// database connections.
use sqlx::Pool;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

/// Importing the model for
/// users for explicit typing.
use crate::modules::models::CleoUser;

/// A structure containing
/// a pool of database connections
/// to make app data persist.
pub struct AppData {
    pub pool: Pool<Postgres>
}

/// Implementing generic
/// methods for the "AppData"
/// structure.
impl AppData{

    /// Implementing a method
    /// to create a new instance
    /// of the "AppData"
    /// structure.
    pub fn new(pg_pool: &Pool<Postgres>) -> AppData{
        AppData { pool: pg_pool.to_owned() }
    }

}

/// A data structure
/// to hold all the
/// neccessary config
/// variables.
pub struct Config{
    pub local_host: String,
    pub local_port: String,
    pub hostname: String,
    pub instance_name: String,
    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_pass: String,
    pub admin_username: String,
    pub admin_email: String,
    pub admin_password: String,
    pub admin_display_name: String,
    pub postgres_user: String,
    pub postgres_port: String,
    pub postgres_host: String,
    pub postgres_pass: String,
    pub file_storage_dir: String
}

/// A data structure
/// holding all information
/// neccessar for creating
/// the first user and 
/// encapsulating that
/// data.
pub struct AdminInfo{
    pub db_url: String,
    pub app_addr: String,
    pub pool: Pool<Postgres>,
    pub admin: CleoUser,
    pub instance: usize

}
