// Declare modules
mod api;
mod components;
mod config;
mod error;
mod utils;

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut auth_status = use_signal(|| false);
    let mut response_text = use_signal(|| String::from("Click 'Check Status' to begin"));

    // Check auth status on mount
    use_effect(move || {
        spawn(async move {
            api::auth::check_auth_status(auth_status, response_text).await;
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
                            api::auth::login().await;
                        });
                    },
                    "Login with Spotify"
                }
                button {
                    onclick: move |_| {
                        spawn(async move {
                            api::auth::check_auth_status(auth_status, response_text).await;
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
