//! Error types and handling for the client

use reqwest::Error as ReqwestError;

/// User-friendly error messages for the client
#[derive(Debug, Clone)]
pub enum ClientError {
    Connection(String),
    Timeout,
    ServerError(u16, String),
    ParseError(String),
    Unknown(String),
}

impl ClientError {
    /// Convert a reqwest error into a user-friendly ClientError
    pub fn from_reqwest(error: ReqwestError) -> Self {
        if error.is_connect() {
            ClientError::Connection(
                "Cannot reach the server. Make sure it's running:\n\
                 → cargo run -p harmony-server"
                    .to_string(),
            )
        } else if error.is_timeout() {
            ClientError::Timeout
        } else if error.is_status() {
            let status = error.status().map(|s| s.as_u16()).unwrap_or(500);
            ClientError::ServerError(status, "The server returned an error response".to_string())
        } else {
            ClientError::Unknown(format!("{}", error))
        }
    }

    /// Format error with emoji and helpful message
    pub fn display_message(&self) -> String {
        match self {
            ClientError::Connection(msg) => {
                format!("🔌 Connection Error\n\n{}", msg)
            }
            ClientError::Timeout => "⏱️ Request Timeout\n\n\
                 The server took too long to respond.\n\
                 Please try again."
                .to_string(),
            ClientError::ServerError(status, msg) => {
                format!("⚠️ Server Error ({})\n\n{}", status, msg)
            }
            ClientError::ParseError(msg) => {
                format!("⚠️ Parse Error\n\n{}", msg)
            }
            ClientError::Unknown(msg) => {
                format!("❌ Unknown Error\n\n{}", msg)
            }
        }
    }
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_message())
    }
}
