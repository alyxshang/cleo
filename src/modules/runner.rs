/*
Mocha Backend by Alyx Shang.
Licensed under the FSL v1.
*/

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

/// Importing the "Config"
/// structure for explicit
/// typing.
use super::units::Config;

/// Importing the "AadminInfo"
/// structure for explicit
/// typing.
use super::units::AdminInfo;

/// Importing all service
/// functions to create
/// extra content fields.
use crate::modules::services::ecf::*;

// Importing all service functions
// to create user keys.
use crate::modules::services::keys::*;

/// Importing all service functions 
/// to work with user-uploadable files.
use crate::modules::services::files::*;

/// Importing all service functions 
/// for people to verify their email 
/// addresses.
use crate::modules::services::email::*;

/// Importing all service functions
/// for creating and deleing users 
/// and updating their information.
use crate::modules::services::users::*;

/// Importing all service functions 
/// with routes relevant for admins.
use crate::modules::services::admin::*;

/// Importing all service functions for
/// creating, updating, and deleting
/// user posts.
use crate::modules::services::posts::*;

/// Importing all service functions for
/// creating and deleting API tokens.
use crate::modules::services::tokens::*;

/// Importing all service functions for
/// retrieving info on different things.
use crate::modules::services::general::*;

/// Importing the "DefaultHeaders" structure
/// to set custom headers.
use actix_web::middleware::DefaultHeaders;

/// Importing the function to read the
/// neccessary data from the enviroment.
use crate::modules::config::create_config;

/// Importing the function to create the
/// neccessary entities for the app.
use crate::modules::config::create_admin_info;

/// Attempts to run the app with some environment
/// variables set. If this operations fails,
/// an error is returned.
pub async fn run_app() -> Result<(), CleoErr> {
    let config: Config = match create_config(){
        Ok(config) => config,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let admin_info: AdminInfo = match create_admin_info(&config).await{
        Ok(config) => config,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if admin_info.admin.username == config.admin_username && admin_info.instance == 0 {
        let data: Data<AppData> = Data::new(AppData::new(&admin_info.pool));
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
                .service(update_email_service)
                .service(update_username_service)
                .service(update_name_service)
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
                .service(edit_instance_name_service)
                .service(edit_instance_hostname_service)
                .service(edit_smtp_server_service)
                .service(edit_smtp_username_service)
                .service(edit_smtp_password_service)
                .service(verify_email_service)
                .service(create_user_file_service)
                .service(static_file_service)
            }
        ).bind(admin_info.app_addr){
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
