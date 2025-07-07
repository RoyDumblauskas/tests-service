use dioxus::prelude::*;

static CSS: Asset = asset!("/src/components/Home/Home.css");

#[component]
pub fn Home() -> Element {
    rsx! { h1 { "Home" } }
}
