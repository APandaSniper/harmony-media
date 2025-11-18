//! Application configuration

/// Get the API base URL
pub fn api_url() -> String {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::env::var("API_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string())
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        // In WASM, you might want to read from window.location or a config object
        "http://localhost:3000".to_string()
    }
}