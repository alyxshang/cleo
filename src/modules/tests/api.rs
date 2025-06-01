/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "App"
/// structure to create a new
/// Actix Web app.
use actix_web::App;

/// Importing all data structures
/// for sending payloads.
use crate::modules::payloads::*:

/// Importing all data structures
/// for checking responses.
use crate::modules::responses::*;

/// Importing all service functions
/// for the ecf-related
/// services.
use crate::modules::services::ecf::*;

/// Importing all service functions
/// for the key-related
/// services.
use crate::modules::services::keys::*;

/// Importing all service functions
/// for the administrator-related
/// services.
use crate::modules::services::admin::*;

/// Importing all service functions
/// for the post-related
/// services.
use crate::modules::services::posts::*;

/// Importing all service functions
/// for the email-related
/// services.
use crate::modules::services::email::*;

/// Importing all service functions
/// for the user-related
/// services.
use crate::modules::services::users::*;

/// Importing all service functions
/// for the file-related
/// services.
use crate::modules::services::files::*;

/// Importing all service functions
/// for the token-related
/// services.
use crate::modules::services::tokens::*;

/// Importing all service functions
/// for general services.
use crate::modules::services::general::*;

/*
## TODO
- Add tests for email verif.
- Add tests for files.
*/

/// The function to test the
/// user service functions.
#[actix_test::test]
pub async fn test_user_service_functions(){
    use actix_web::test;
    let app = test::init_service(
        App::new()
            .service(create_user_service)
            .service(update_email_service)
            .service(update_username_service)
            .service(update_name_service)
            .service(update_pfp_service)
            .service(update_password_service)
            .service(delete_user_service)
        )
        .await;
    let req_create_user = test::TestRequest::post().uri("/user/create")
            .insert_header(ContentType::json())
            .set_json(
                UserCreationPayload{ 
                    username: "alyshang",
                    display: "Aly Shang",
                    password: "12345678",
                    email_addr: "example@example.com",
                    pfp_url: "https://avatars.githubusercontent.com/u/6471485?s=200&v=4",
                    user_key: "1234567890",
                }
            )
            .to_request();
    let resp_create_user = test::call_service(&app, req_create_user).await;
    let req_create_token = test::TestRequest::post().uri("/token/create")
            .insert_header(ContentType::json())
            .set_json(
                AuthActionPayload{ 
                    username: "alyshang", 
                    password: "12345678" 
                }
            )
            .to_request();
    let resp_create_token = test::call_service(&app, req_create_token).await;
    let token: String resp_create_token.token;
    let req_u_email = test::TestRequest::post().uri("/user/update/email")
            .insert_header(ContentType::json())
            .set_json(
                UserChangePayload{ 
                    api_token: token,
                    new_value: "ziggy@stardust.com",
                }
            )
            .to_request();
    let resp_u_email = test::call_service(&app, req_u_email).await;
    let req_u_uname = test::TestRequest::post().uri("/user/update/username")
            .insert_header(ContentType::json())
            .set_json(
                UserChangePayload{ 
                    api_token: token,
                    new_value: "alyxshang",
                }
            )
            .to_request();
    let resp_u_uname = test::call_service(&app, req_u_uname).await;
    let req_u_name = test::TestRequest::post().uri("/user/update/name")
            .insert_header(ContentType::json())
            .set_json(
                UserChangePayload{ 
                    api_token: token,
                    new_value: "Alyx Shang",
                }
            )
            .to_request();
    let resp_u_name = test::call_service(&app, req_u_name).await;
    let req_u_password = test::TestRequest::post().uri("/user/update/password")
            .insert_header(ContentType::json())
            .set_json(
                UserChangePayload{ 
                    api_token: token,
                    new_value: "12340987",
                }
            )
            .to_request();
    let resp_u_password = test::call_service(&app, req_u_password).await;
    let req_u_pic = test::TestRequest::post().uri("/user/update/picture")
            .insert_header(ContentType::json())
            .set_json(
                UserChangePayload{ 
                    api_token: token,
                    new_value: "https://avatars.githubusercontent.com/u/179976644?v=4",
                }
            )
            .to_request();
    let resp_u_pic = test::call_service(&app, req_u_pic).await;
    let req_del_user = test::TestRequest::post().uri("/user/delete")
            .insert_header(ContentType::json())
            .set_json(
                AuthActionPayload{ 
                    username: "alyxshang",
                    password: "12340987"
                }
            )
            .to_request();
    let resp_del_user = test::call_service(&app, req_del_user).await;
    assert!(resp_create_user.status().is_success());
    assert!(resp_u_uname.status().is_success());
    assert!(resp_u_name.status().is_success());
    assert!(resp_u_pic.status().is_success());
    assert!(resp_u_email.status().is_success());
    assert!(resp_del_user.status().is_success());
}

/// The function to test the
/// API token service functions.
#[actix_web::test]
pub async fn test_api_token_services() {
    use actix_web::test;
    let app = test::init_service(
        App::new()
            .service(create_api_token_service)
            .service(delete_api_token_service)
        )
        .await;
        let req_create = test::TestRequest::post().uri("/token/create")
            .insert_header(ContentType::json())
            .set_json(
                AuthActionPayload{ 
                    username: "alyx", 
                    password: "12345678" 
                }
            )
            .to_request();
        let resp_create = test::call_service(&app, req_create).await;
        let token: String = resp_create.token;
        let req_delete = test::TestRequest::post().uri("/token/delete")
            .insert_header(ContentType::json())
            .set_json(
                DelTokenPayload{ 
                    token: token,
                    username: "alyx", 
                    password: "12345678" 
                }
            )
            .to_request();
        assert!(resp_create.status().is_success());
        assert!(resp_delete.status().is_success());
}

/// The function to tet the
/// user-posts service functions.
#[actix_web::test]
pub async fn test_posts_services(){
    use actix_web::test;
    let app = test::init_service(
        App::new()
            .service(create_user_post_service)
            .service(update_user_post.service)
    ).await;
    let req_token_create = test::TestRequest::post().uri("/token/create")
            .insert_header(ContentType::json())
            .set_json(
                AuthActionPayload{ 
                    username: "alyx", 
                    password: "12345678" 
                }
            )
            .to_request();
    let resp_token_create = test::call_service(&app, req_token_create).await;
    let token: String = resp_token_create.token;
    let req_post_create = test::TestRequest::post().uri("/posts/create")
            .insert_header(ContentType::json())
            .set_json(
                PostCreationPayload{ 
                    api_token: token, 
                    content_type: "page",
                    content_text: "<h1>Hello World</h1>"
                }
            )
            .to_request();
    let resp_post_create = test::call_service(&app, req_post_create).await;
    let id: String = resp_post_create.content_id;
    let req_post_update = test::TestRequest::post().uri("/posts/update")
            .insert_header(ContentType::json())
            .set_json(
                UpdatePostPayload{ 
                    api_token: token, 
                    content__id: id,
                    text: "<h1>Hello World haha</h1>"
                }
            )
            .to_request();
    let resp_post_update = test::call_service(&app, req_post_update).await;
    let req_post_delete = test::TestRequest::post().uri("/posts/delete")
            .insert_header(ContentType::json())
            .set_json(
                DeletePostPayload{ 
                    api_token: token, 
                    content__id: id,
                }
            )
            .to_request();
    let resp_post_delete = test::call_service(&app, req_post_delete).await;
    assert!(resp_post_create.status().is_success());
    assert!(resp_post_update.status().is_success());
    assert!(resp_post_delete.status().is_success());
}

/// A function to test the 
/// service functions for
/// user keys.
#[actix_web::test]
pub async fn test_user_key_services(){
   let app = test::init_service(
        App::new()
            .service(create_user_post_service)
            .service(update_user_post.service)
    ).await;
    let req_token_create = test::TestRequest::post().uri("/token/create")
            .insert_header(ContentType::json())
            .set_json(
                userKeyPayload{ 
                    username: "alyx", 
                    password: "12345678" 
                }
            )
            .to_request();
    let resp_token_create = test::call_service(&app, req_token_create).await;
    let token: String = resp_token_create.token;
    let req_key_create = test::TestRequest::post().uri("/keys/create")
            .insert_header(ContentType::json())
            .set_json(
                UserKeyPayload{ 
                    key_type: "admin",
                    api_token: token,
                    username: "alyx",
                }
            )
            .to_request();
    let resp_key_create = test::call_service(&app, req_key_create).await;
    let id: String = resp_key_create.key_id;
    let req_key_delete = test::TestRequest::post().uri("/keys/delete")
            .insert_header(ContentType::json())
            .set_json(
                UserKeyDeletionPayload{ 
                    api_token: token,
                    key_id: id,
                }
            )
            .to_request();
    let resp_key_delete = test::call_service(&app, req_key_delete).await;
    let req_keys_get = test::TestRequest::post().uri("/keys/all")
            .insert_header(ContentType::json())
            .set_json(
                TokenOnlyPayload{ 
                    api_token: token,
                }
            )
            .to_request();
    let resp_keys_get = test::call_service(&app, req_keys_get).await;
    assert!(resp_key_create.status().is_success());
    assert!(resp_key_delete.status().is_success());
    assert!(resp_keys_get.status().is_success());
}

/// A function to test general
/// API service functions.
#[actix::test]
pub async fn test_general_services(){
    let app = test::init_service(
        App::new()
            .service(get_user_posts_service)
            .service(get_user_files_service)
        ).await;
    let req_token_create = test::TestRequest::post().uri("/token/create")
            .insert_header(ContentType::json())
            .set_json(
                userKeyPayload{ 
                    username: "alyx", 
                    password: "12345678" 
                }
            )
            .to_request();
    let resp_token_create = test::call_service(&app, req_token_create).await;
    let token: String = resp_token_create.token;
    let req_posts_get = test::TestRequest::post().uri("/posts/all")
            .insert_header(ContentType::json())
            .set_json(
                TokenOnlyPayload{ 
                    api_token: token,
                }
            )
            .to_request();
    let resp_posts_get = test::call_service(&app, req_posts_get).await;
    let req_files_get = test::TestRequest::post().uri("/files/all")
            .insert_header(ContentType::json())
            .set_json(
                TokenOnlyPayload{ 
                    api_token: token,
                }
            )
            .to_request();
    let resp_files_get = test::call_service(&app, req_files_get).await;
    assert!(resp_posts_get.status().is_success());
    assert!(resp_files_get.status().is_success());
}

/// Testing the service functions for 
/// working with extra content fields
/// on user posts.
#[actix_web::test]
pub async fn test_ecf_service_functions(){
    use actix_web::test;
    let app = test::init_service(
        App::new()
            .service(create_user_post_service)
            .service(update_user_post.service)
    ).await;
    let req_token_create = test::TestRequest::post().uri("/token/create")
            .insert_header(ContentType::json())
            .set_json(
                AuthActionPayload{ 
                    username: "alyx", 
                    password: "12345678" 
                }
            )
            .to_request();
    let resp_token_create = test::call_service(&app, req_token_create).await;
    let token: String = resp_token_create.token;
    let req_post_create = test::TestRequest::post().uri("/posts/create")
            .insert_header(ContentType::json())
            .set_json(
                PostCreationPayload{ 
                    api_token: token, 
                    content_type: "page",
                    content_text: "<h1>Hello World</h1>"
                }
            )
            .to_request();
    let resp_post_create = test::call_service(&app, req_post_create).await;
    let id: String = resp_post_create.content_id;
    let req_ecf_create = test::TestRequest::post().uri("/ecf/create")
            .insert_header(ContentType::json())
            .set_json(
                ExtraContentFieldCreationPayload{ 
                    api_token: token, 
                    content_id: id,
                    field_key: "name",
                    field_value: "Hello World!"
                }
            )
            .to_request();
    let resp_ecf_create = test::call_service(&app, req_ecf_create).await;
    let req_ecf_k_update = test::TestRequest::post().uri("/ecf/edit/key")
            .insert_header(ContentType::json())
            .set_json(
                EditExtraContentFieldPayload{ 
                    api_token: token, 
                    content_id: id,
                    field_id: resp_ecf_create.field_id,
                    new_key: "title"
                }
            )
            .to_request();
    let resp_ecf_k_update = test::call_service(&app, req_ecf_k_update).await;
    let req_ecf_v_update = test::TestRequest::post().uri("/ecf/edit/value")
            .insert_header(ContentType::json())
            .set_json(
                EditExtraContentFieldPayload{ 
                    api_token: token, 
                    content_id: id,
                    field_id: resp_ecf_create.field_id,
                    new_value: "Hello"
                }
            )
            .to_request();
    let resp_ecf_v_update = test::call_service(&app, req_ecf_v_update).await;
    let req_ecf_delete = test::TestRequest::post().uri("/ecf/delete")
            .insert_header(ContentType::json())
            .set_json(
                DeleteExtraContentFieldPayload{ 
                    api_token: token, 
                    content_id: id,
                    field_id: resp_ecf_create.field_id,
                }
            )
            .to_request();
    let resp_ecf_delete = test::call_service(&app, req_ecf_delete).await;
    assert!(resp_ecf_create.status().is_success());
    assert!(resp_ecf_v_update.status().is_success());
    assert!(resp_ecf_k_update.status().is_success());
    assert(resp_ecf_delete.status().is_success());
}

/// Testing the service functions
/// for the administrators.
#[actix_web::test]
pub async fn test_admin_service_functions(){
    use actix_web::test;
    let app = test::init_service(
        App::new()
            .service(create_user_post_service)
            .service(update_user_post.service)
    ).await;
    let req_token_create = test::TestRequest::post().uri("/token/create")
            .insert_header(ContentType::json())
            .set_json(
                AuthActionPayload{ 
                    username: "alyx", 
                    password: "12345678" 
                }
            )
            .to_request();
    let resp_token_create = test::call_service(&app, req_token_create).await;
    let token: String = resp_token_create.token;
    let req_admins_get = test::TestRequest::post().uri("/instance/admins")
            .insert_header(ContentType::json())
            .set_json(
                TokenOnlyPayload{ 
                    api_token: token, 
                }
            )
            .to_request();
    
    let resp_admins_get = test::call_service(&app, req_admins_get).await;
    let req_users_get = test::TestRequest::post().uri("/instance/users")
            .insert_header(ContentType::json())
            .set_json(
                TokenOnlyPayload{ 
                    api_token: token, 
                }
            )
            .to_request();
    
    let resp_users_get = test::call_service(&app, req_users_get).await;
    let req_i_name = test::TestRequest::post().uri("/instance/edit/name")
            .insert_header(ContentType::json())
            .set_json(
                UserChangePayload{ 
                    api_token: token, 
                    new_value: "KitsuClub"
                }
            )
            .to_request();
    
    let resp_i_name = test::call_service(&app, req_i_name).await;
    let req_i_hname = test::TestRequest::post().uri("/instance/edit/hostname")
            .insert_header(ContentType::json())
            .set_json(
                UserChangePayload{ 
                    api_token: token, 
                    new_value: "kitsublog.blog"
                }
            )
            .to_request();
    
    let resp_i_hname = test::call_service(&app, req_i_hname).await;
    let req_i_ss = test::TestRequest::post().uri("/instance/edit/smtp/server")
            .insert_header(ContentType::json())
            .set_json(
                UserChangePayload{ 
                    api_token: token, 
                    new_value: "kitsublog.blog"
                }
            )
            .to_request();
    
    let resp_i_ss = test::call_service(&app, req_i_ss).await;
    let req_i_su = test::TestRequest::post().uri("/instance/edit/smtp/username")
            .insert_header(ContentType::json())
            .set_json(
                UserChangePayload{ 
                    api_token: token, 
                    new_value: "kitsuadmin"
                }
            )
            .to_request();
    
    let resp_i_su = test::call_service(&app, req_i_su).await;
    let req_i_sp = test::TestRequest::post().uri("/instance/edit/smtp/pass")
            .insert_header(ContentType::json())
            .set_json(
                UserChangePayload{ 
                    api_token: token, 
                    new_value: "kitsuadmin"
                }
            )
            .to_request();
    
    let resp_i_sp = test::call_service(&app, req_i_sp).await;
    assert!(resp_admins_get.status().is_success());
    assert!(resp_users_get.status().is_success());
    assert!(resp_i_name.status().is_success());
    assert!(resp_i_hname.status().is_success());
    assert!(resp_i_name.status().is_success());
    assert!(resp_i_ss.status().is_success());
    assert!(resp_i_su.status().is_success());
    assert!(resp_i_sp.status().is_success());
}
