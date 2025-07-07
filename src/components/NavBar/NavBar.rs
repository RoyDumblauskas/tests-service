use dioxus::prelude::*;

static CSS: Asset = asset!("/src/components/NavBar/NavBar.css");

#[component]
pub fn NavBar() -> Element {
    rsx! { h1 { "text" } }
}
