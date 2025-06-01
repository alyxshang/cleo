/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the function
/// to run the app.
use cleo::run_app;

/// Main point of
/// entry for the 
/// Rust compiler.
#[actix_web::main]
async fn main(){
    match run_app().await {
        Ok(_feedback) => {},
        Err(e) => eprintln!("{}", &e.to_string())
    }
}