/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the 
/// "Deserialize" trait
/// to derive it.
use serde::Deserialize;

/// A data structure
/// to extract the filename
/// from file request URLs.
#[derive(Deserialize)]
pub struct FileExtractor{
    pub filename: String
}

/// Declaring a structure
/// to extract the "token"
/// parameter from an URL.
#[derive(Deserialize)]
pub struct TokenExtractor{
    pub token: String
}
