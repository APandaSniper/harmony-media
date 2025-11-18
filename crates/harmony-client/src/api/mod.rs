//! API client modules

pub mod auth;
pub mod spotify;

// Re-export commonly used items
pub use auth::{check_auth_status, login};