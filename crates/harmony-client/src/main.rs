use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            h1 { "Harmony Media" }
            p { "Welcome to your music quiz game!" }
        }
    }
}
