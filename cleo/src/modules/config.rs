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
/// function retrieve
/// environment
/// variables for initial
/// settings.
use std::env::var;

/// Importing this crate's
/// error structure.
use super::err::CleoErr;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

/// Importing the data structure
/// that holds all the neccessary
/// information for getting a 
/// Cleo instance up and running.
use crate::modules::units::Config;

/// Importing the model for
/// users for explicit typing.
use crate::modules::models::CleoUser;

/// Importing the data structure
/// to hold all neccessar for 
/// Cleo lift-off.
use crate::modules::units::AdminInfo;

/// Importing the function to write a new
/// Cleo user to the database.
use crate::modules::db::users::create_user;

/// Importing the function to establish a
/// connection pool with the Postgres database.
use crate::modules::utils::create_connection;

/// Importing the function to write a new
/// Cleo instance to the database.
use crate::modules::db::admin::create_instance_info;

/// This function attempts to read all environment
/// variables from the environment required for
/// setting up a Cleo instance. If the operation
/// is successful, an instance of the "Config"
/// structure is returned. If the operation fails,
/// an error is returned.
pub fn create_config() -> Result<Config, CleoErr>{
    let local_host: String = match var("CLEO_HOST"){
        Ok(local_host) => local_host,
        Err(e) => return Err::<Config, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let local_port: String = match var("CLEO_PORT"){
        Ok(local_port) => local_port,
        Err(e) => return Err::<Config, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let hostname: String = match var("CLEO_HOSTNAME"){
        Ok(hostname) => hostname,
        Err(e) => return Err::<Config, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let instance_name: String = match var("CLEO_INSTANCE_NAME"){
        Ok(instance_name) => instance_name,
        Err(e) => return Err::<Config, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let smtp_server: String = match var("CLEO_SMTP_SERVER"){
        Ok(smtp_server) => smtp_server,
        Err(e) => return Err::<Config, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let smtp_username: String = match var("CLEO_SMTP_USERNAME"){
        Ok(smtp_username) => smtp_username,
        Err(e) => return Err::<Config, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let smtp_pass: String = match var("CLEO_SMTP_PASS"){
        Ok(smtp_pass) => smtp_pass,
        Err(e) => return Err::<Config, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let admin_username: String = match var("CLEO_ADMIN_USERNAME"){
        Ok(admin_username) => admin_username,
        Err(e) => return Err::<Config, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let admin_email: String = match var("CLEO_ADMIN_EMAIL"){
        Ok(admin_email) => admin_email,
        Err(e) => return Err::<Config, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let admin_password: String = match var("CLEO_ADMIN_PASSWORD"){
        Ok(admin_password) => admin_password,
        Err(e) => return Err::<Config, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let admin_display_name: String = match var("CLEO_ADMIN_PASSWORD"){
        Ok(admin_display_name) => admin_display_name,
        Err(e) => return Err::<Config, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let postgres_user: String = match var("CLEO_POSTGRES_USER"){
        Ok(postgres_user) => postgres_user,
        Err(e) => return Err::<Config, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let postgres_port: String = match var("CLEO_POSTGRES_PORT"){
        Ok(postgres_port) => postgres_port,
        Err(e) => return Err::<Config, CleoErr>(CleoErr::new(&e.to_string()))
    };    let postgres_host: String = match var("CLEO_POSTGRES_HOST"){
        Ok(postgres_host) => postgres_host,
        Err(e) => return Err::<Config, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let postgres_pass: String = match var("CLEO_POSTGRES_PASS"){
        Ok(postgres_pass) => postgres_pass,
        Err(e) => return Err::<Config, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let file_dir: String = match var("CLEO_FILE_DIR"){
        Ok(file_dir) => file_dir,
        Err(e) => return Err::<Config, CleoErr>(CleoErr::new(&e.to_string()))
    };    let config: Config = Config{
        local_host: local_host,
        local_port: local_port,
        hostname: hostname,
        instance_name: instance_name,
        smtp_server: smtp_server,
        smtp_username: smtp_username,
        smtp_pass: smtp_pass,
        admin_username: admin_username,
        admin_email: admin_email,
        admin_password: admin_password,
        admin_display_name: admin_display_name,
        postgres_user: postgres_user,
        postgres_host: postgres_host,
        postgres_port: postgres_port,
        postgres_pass: postgres_pass,
        file_storage_dir: file_dir
    };
    Ok(config)
}

/// This function attempts to build all
/// neccessary entities for creating
/// a new Cleo instance. If this operation
/// is successful, an instance of the "AdminInfo"
/// structure is returned. If the operation fails,
/// an error is returned.
pub async fn create_admin_info(config: &Config) -> Result<AdminInfo, CleoErr>{
    let db_url: String = format!(
        "postgres://{}:{}@{}:{}/cleo", 
        config.postgres_user, 
        config.postgres_pass, 
        config.postgres_host,
        config.postgres_port
    );
    let app_addr: String = format!(
        "{}:{}", 
        config.local_host, 
        config.local_port
    );
    let connection: Pool<Postgres> = match create_connection(&db_url).await{
        Ok(connection) => connection,
        Err(e) => return Err::<AdminInfo, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let admin_user: CleoUser = match create_user(
        &config.admin_username,
        &config.admin_display_name,
        &config.admin_password,
        &config.admin_email,
        &"".to_string(),
        &"".to_string(),
        &connection
    ).await {
        Ok(admin_user) => admin_user,
        Err(e) => return Err::<AdminInfo, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let instance: usize = match create_instance_info(
        &connection, 
        &config.smtp_server, 
        &config.hostname, 
        &config.instance_name, 
        &config.smtp_username, 
        &config.smtp_pass,
        &config.file_storage_dir
    ).await {
        Ok(instance) => instance,
        Err(e) => return Err::<AdminInfo, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let res: AdminInfo = AdminInfo{
        db_url: db_url,
        app_addr: app_addr,
        pool: connection,
        admin: admin_user,
        instance: instance
    };
    Ok(res)
}
