/*
Mocha Backend by Alyx Shang.
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

/// Importing the "App"
/// structure to create a new
/// Actix Web app.
use actix_web::App;

/// Importing the "Cors"
/// structure to add CORS
/// rules.
use actix_cors::Cors;

/// Importing the "Data"
/// structure to register
/// persistent app data.
use actix_web::web::Data;

/// Importing this crate's
/// error structure.
use super::err::CleoErr;

/// Importing the "HttpServer"
/// structure to create an
/// Actix Web app.
use actix_web::HttpServer;

/// Importing the "AppData"
/// structure to register
/// persistent app data.
use super::units::AppData;

/// Importing the model for
/// users for explicit typing.
use super::models::CleoUser;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

/// Importing all
/// API service functions.
use crate::modules::services::*;

/// Importing the "create_connection"
/// function to create a connection
/// to the PostgreSQL database.
use super::utils::create_connection;

/// Importing the "DefaultHeaders" structure
/// to set custom headers.
use actix_web::middleware::DefaultHeaders;

/// Attempts to run the app with some environment
/// variables set. If this operations fails,
/// an error is returned.
pub async fn run_app() -> Result<(), CleoErr> {
    let local_host: String = match var("CLEO_HOST"){
        Ok(local_host) => local_host,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let local_port: String = match var("CLEO_PORT"){
        Ok(local_port) => local_port,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let hostname: String = match var("CLEO_HOSTNAME"){
        Ok(hostname) => hostname,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let instance_name: String = match var("CLEO_INSTANCE_NAME"){
        Ok(instance_name) => instance_name,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let smtp_server: String = match var("CLEO_SMTP_SERVER"){
        Ok(smtp_server) => smtp_server,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let smtp_username: String = match var("CLEO_SMTP_USERNAME"){
        Ok(smtp_username) => smtp_username,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let smtp_pass: String = match var("CLEO_SMTP_PASS"){
        Ok(smtp_pass) => smtp_pass,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let admin_username: String = match var("CLEO_ADMIN_USERNAME"){
        Ok(admin_username) => admin_username,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let admin_email: String = match var("CLEO_ADMIN_EMAIL"){
        Ok(admin_email) => admin_email,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let admin_password: String = match var("CLEO_ADMIN_PASSWORD"){
        Ok(admin_password) => admin_password,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let admin_display_name: String = match var("CLEO_ADMIN_PASSWORD"){
        Ok(admin_display_name) => admin_display_name,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let postgres_user: String = match var("CLEO_POSTGRES_USER"){
        Ok(postgres_user) => postgres_user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let postgres_host: String = match var("CLEO_POSTGRES_HOST"){
        Ok(postgres_host) => postgres_host,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let postgres_pass: String = match var("CLEO_POSTGRES_PASS"){
        Ok(postgres_pass) => postgres_pass,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let db_url: String = format!(
        "postgres://{}:{}@{}/cleo", 
        postgres_user, 
        postgres_pass, 
        postgres_host
    );
    let app_addr: String = format!("{}:{}", local_host, local_port);
    let connection: Pool<Postgres> = match create_connection(&db_url).await{
        Ok(connection) => connection,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let admin_user: CleoUser = match create_user(
        &admin_username,
        &admin_display_name,
        &admin_password,
        &admin_email,
        &"".to_string(),
        &"".to_string(),
        &connection
    ).await {
        Ok(admin_user) => admin_user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let instance: usize = match create_instance_info(
        &connection, 
        &smtp_server, 
        &hostname, 
        &instance_name, 
        &smtp_username, 
        &smtp_pass
    ).await {
        Ok(instance) => instance,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if admin_user.username == admin_username && instance == 0 {
        let data: Data<AppData> = Data::new(AppData::new(&connection));
        let server = match HttpServer::new(
        move || {
            let cors = Cors::permissive()
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST"]);
            App::new()
                .wrap(cors)
                .wrap(DefaultHeaders::new()
                    .add(("Access-Control-Allow-Origin", "*"))
                    .add(("Access-Control-Allow-Methods", "GET,POST"))
                    .add(("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept"))
                )
                .app_data(data.clone())
                .service(create_user_service)
                .service(update_username_service)
                .service(update_name_service)
                .service(update_email_service)
                .service(update_pfp_service)
                .service(update_password_service)
                .service(delete_user_service)
                .service(create_api_token_service)
                .service(delete_api_token_service)
                .service(create_user_post_service)
                .service(update_user_post_service)
                .service(delete_user_post_service)
                .service(create_extra_content_field_service)
                .service(edit_extra_content_field_key_service)
                .service(edit_extra_content_field_value_service)
                .service(delete_extra_content_field_service)
                .service(delete_user_file_service)
                .service(create_user_key_service)
                .service(delete_user_key_service)
                .service(get_instance_admins_service)
                .service(get_instance_users_service)
                .service(get_user_keys_service)
                .service(get_user_files_service)
                .service(get_user_posts_service)
                .service(get_instance_info_service)
                .service(edit_instance_name_service)
                .service(edit_instance_hostname_service)
                .service(edit_smtp_server_service)
                .service(edit_smtp_username_service)
                .service(edit_smtp_password_service)
            }
        ).bind(app_addr){
            Ok(server) => server,
            Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
        };
        let running: () = match server.run().await{
            Ok(running) => running,
            Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
        };
        Ok(running)
    }
    else {
        let e: String = format!("Initial data could not be written.");
        return Err::<(), CleoErr>(CleoErr::new(&e.to_string()));
    }
}
