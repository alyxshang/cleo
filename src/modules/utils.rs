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

/// Importing the "Digest"
/// trait from the "sha2"
/// crate.
use sha2::Digest;

/// Importing the "Sha256"
/// structure from the "sha2"
/// crate to process strings.
use sha2::Sha256;

/// Importing the structure
/// from the "lettre" crate
/// to send an email.
use lettre::Message;

/// Importing the prelude from
/// the "rand" crate to get
/// random items from a vector.
use rand::prelude::*;

/// Importing this crate's
/// error structure.
use super::err::CleoErr;

/// Using the "Local"
/// structure from the "chrono"
/// crate to retrieve the current 
/// time.
use chrono::offset::Local;

/// Importing the executor
/// for the asynchronous
/// runtime over which
/// to send an email.
use lettre::Tokio1Executor;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

/// Importing the structure
/// to enable the asynchronous
/// sending of emails.
use lettre::transport::AsyncTransport;

/// Importing the structure to send SMTP
/// messages asynchronously.
use lettre::transport::smtp::AsyncSmtpTransport;

/// Importing the structure to receive responses
/// from sending SMTP messages.
use lettre::transport::smtp::response::Response;

/// Importing the structure for supplying credentials
/// to "lettre".
use lettre::transport::smtp::authentication::Credentials;

/// Creates and returns the SHA-256 sum
/// of the supplied string.
pub fn hash_string(subject: &String) -> String {
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(subject);
    format!("{:X}", hasher.finalize())
}

/// Generates the ID of every link submitted
/// by taking the last four characters of a 
/// SHA-256 sum.
pub fn generate_id(shasum: &String) -> String {
    let shasum_chars: Vec<char> = shasum.chars().collect();
    let start: usize = shasum_chars.len() - 4;
    let end: usize = shasum_chars.len();
    let mut result_vec: Vec<String> = Vec::new();
    for i in start..end {
        result_vec.push(shasum_chars[i].to_string());
    }
    let result: String = result_vec.join("");
    result
}

/// Attempts to create a connection to a PostgreSQL database given a database
/// URL. If this operation fails, an error is returned.
pub async fn create_connection(db_url: &String) -> Result<Pool<Postgres>, CleoErr> {
    let conn = match sqlx::postgres::PgPool::connect(db_url).await{
        Ok(conn) => conn,
        Err(e) => return Err::<Pool<Postgres>, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(conn)
}

/// A generic structure to
/// hold information on the current
/// local time.
pub struct TimeNow{
    pub year: String,
    pub month: String,
    pub day: String,
    pub hours: String,
    pub minutes: String,
    pub seconds: String,
    pub millis: String
}

/// Implementing generic
/// methods for the "TimeNow"
/// structure.
impl TimeNow{

    /// Implementing a "new"
    /// method for the "TimeNow"
    /// structure.
    pub fn new() -> TimeNow {
        let time = Local::now();
        let date = time.date_naive();
        let curr_time = time.time();
        let year: String = format!("{}",date.format("%Y"));
        let month: String = format!("{}",date.format("%m"));
        let day: String = format!("{}",date.format("%d"));
        let hours: String = format!("{}",curr_time.format("%H"));
        let minutes: String = format!("{}",curr_time.format("%M"));
        let seconds: String = format!("{}",curr_time.format("%S"));
        let millis: String = format!("{}",curr_time.format("%f"));
        TimeNow {
            year,
            month,
            day,
            hours,
            minutes,
            seconds,
            millis
        }
    }
    
    /// Implementing a generic function
    /// to return a string representation
    /// of this structure.
    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}",
            &self.year, 
            &self.month, 
            &self.day, 
            &self.hours, 
            &self.minutes, 
            &self.seconds,
            &self.millis
        )
    }
}

/// Attempts to generate a random character
/// sequence of the supplied size. If this 
/// operation fails, an error is returned.
pub fn generate_key(size: &usize) -> Result<String, CleoErr> {
    let mut char_vec: Vec<char> = Vec::new();
    let alpha: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890"
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    for _i in 1..(size+1){
        let mut random: ThreadRng = rand::rng();
        let rnd_char: char = match alpha.choose(&mut random){
            Some(rnd_char) => *rnd_char,
            None => return Err::<String, CleoErr>(CleoErr::new(&"Could not pick random character.".to_string()))
        };
        char_vec.push(
            rnd_char
        );
    }
    Ok(char_vec.into_iter().collect::<String>())
}

/// This function attempts to
/// send an email to the specified
/// sender with the specified parameters.
/// If this operation fails, an error is
/// returned.
pub async fn send_email(
    sender: &String,
    password: &String,
    subject: &String,
    body: &String,
    receiver: &String,
    server: &String
) -> Result<bool, CleoErr> {
    let smtp_credentials = Credentials::new(sender.to_string(), password.to_string());
    let mailer = match AsyncSmtpTransport::<Tokio1Executor>::relay(&server)
    {
        Ok(mailer) => mailer.credentials(smtp_credentials).build(),
        Err(e) => return Err::<bool, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let to = match receiver.parse(){
        Ok(to) => to,
        Err(_e) => return Err::<bool, CleoErr>(CleoErr::new(&"Could not parse receiver.".to_string()))
    };
    let from = match sender.parse(){
        Ok(from) => from,
        Err(_e) => return Err::<bool, CleoErr>(CleoErr::new(&"Could not parse sender.".to_string()))
    };
    let email = match Message::builder()
        .from(from)
        .to(to)
        .subject(subject)
        .body(body.to_string())
    {
        Ok(email) => email,
        Err(e) => return Err::<bool, CleoErr>(CleoErr::new(&e.to_string()))
    };
    let send_op: Response = match mailer.send(email).await {
        Ok(send_op) => send_op,
        Err(e) => return Err::<bool, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(send_op.is_positive())
}
