/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Exporting this
/// crate's tests module.
#[cfg(test)]
pub mod tests;

/// Exporting the
/// module for reading
/// and writing to and
/// from the database.
pub mod db;

/// Exporting the
/// module containing
/// this crate's
/// error-handling
/// structure.
pub mod err;

/// Exporting 
/// a module
/// containing
/// miscallenous
/// data structures.
pub mod units;

/// Exporting the
/// module containing
/// some utility
/// functions.
pub mod utils;

/// Exporting the
/// module containing
/// this crate's
/// error-handling
/// structure.
pub mod models;

/// Exporting the
/// module containing
/// the function that
/// runs the whole
/// app.
pub mod runner;

/// Exporting the module
/// for gathering all
/// the neccessary 
/// variables from
/// the current 
/// environment.
pub mod config;

/// Exporting the
/// module containing
/// data structures
/// for submitting
/// data to different
/// API services.
pub mod payloads;

/// Exporting the
/// module containing
/// this crate's
/// API service
/// functions.
pub mod services;

/// Exporting the 
/// module containing
/// data structures
/// for returning
/// data from different
/// API services.
pub mod responses;

/// Exporting the module
/// containing extractors
/// for request paths.
pub mod extractors;
