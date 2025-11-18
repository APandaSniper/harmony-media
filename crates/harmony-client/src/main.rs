use dioxus::prelude::*;

const API_URL: &str = "http://localhost:3000";

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    dioxus::launch(App);
    
    #[cfg(target_arch = "wasm32")]
    dioxus::web::launch(App);  // Use web launcher for WASM
}

#[component]
fn App() -> Element {
    let mut auth_status = use_signal(|| false);
    let mut response_text = use_signal(|| String::from("Click 'Check Status' to begin"));

    // Check auth status on mount
    use_effect(move || {
        spawn(async move {
            check_auth_status(auth_status, response_text).await;
        });
    });

    rsx! {
        style { {include_str!("../assets/style.css")} }
        div { class: "container",
            h1 { "🎵 Harmony Media Dashboard" }
            
            div { class: "section",
                h2 { "Authentication" }
                button { 
                    onclick: move |_| {
                        spawn(async move {
                            login().await;
                        });
                    },
                    "Login with Spotify" 
                }
                button { 
                    onclick: move |_| {
                        spawn(async move {
                            check_auth_status(auth_status, response_text).await;
                        });
                    },
                    "Check Auth Status" 
                }
                
                div { id: "authStatus",
                    if auth_status() {
                        div { class: "status authenticated",
                            "✅ Authenticated with Spotify"
                        }
                    } else {
                        div { class: "status not-authenticated",
                            "❌ Not authenticated"
                        }
                    }
                }
            }

            div { class: "section",
                h2 { "Response" }
                div { id: "response",
                    "{response_text}"
                }
            }
        }
    }
}

async fn login() {
    let url = format!("{}/auth/login", API_URL);
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = webbrowser::open(&url);
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        // In browser, navigate to the auth URL
        if let Some(window) = web_sys::window() {
            let _ = window.location().set_href(&url);
        }
    }
}

async fn check_auth_status(mut auth_status: Signal<bool>, mut response_text: Signal<String>) {
    let url = format!("{}/auth/status", API_URL);
    
    match reqwest::get(&url).await {
        Ok(response) => {
            match response.json::<serde_json::Value>().await {
                Ok(data) => {
                    auth_status.set(data["authenticated"].as_bool().unwrap_or(false));
                    response_text.set(serde_json::to_string_pretty(&data).unwrap_or_default());
                }
                Err(e) => {
                    response_text.set(format!("Error parsing response: {}", e));
                }
            }
        }
        Err(e) => {
            response_text.set(format!("Error: {}", e));
        }
    }
}