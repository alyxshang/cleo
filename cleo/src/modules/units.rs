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

/// Importing the 
/// "Deserialize" trait
/// to derive it.
use serde::Deserialize;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

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
