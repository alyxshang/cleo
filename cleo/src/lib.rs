/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the
/// module for reading
/// and writing to and
/// from the database.
pub use modules::db::*;

/// Re-exporting the
/// module containing
/// this crate's
/// error-handling
/// structure.
pub use modules::err::*;

/// Re-exporting 
/// a module
/// containing
/// miscallenous
/// data structures.
pub use modules::units::*;

/// Re-exporting the
/// module containing
/// some utility
/// functions.
pub use modules::utils::*;

/// Re-exporting the
/// module containing
/// this crate's
/// error-handling
/// structure.
pub use modules::models::*;

/// Re-exporting the
/// module containing
/// the function that
/// runs the whole
/// app.
pub use modules::runner::*;

/// Exporting the
/// module containing
/// this crate's
/// API service
/// functions.
pub use modules::services::*;

/// Re-exporting the
/// module containing
/// data structures
/// for submitting
/// data to different
/// API services.
pub use modules::payloads::*;

/// Re-exporting the 
/// module containing
/// data structures
/// for returning
/// data from different
/// API services.
pub use modules::responses::*;

/// Re-exporting the module
/// containing extractors
/// for request paths.
pub use modules::extractors::*;
