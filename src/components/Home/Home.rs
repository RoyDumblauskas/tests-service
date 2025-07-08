use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/component-css/Home.css");

#[component]
pub fn Home() -> Element {
    rsx! { 
        document::Stylesheet { href: CSS }
        h1 { "Home" }
    }
}
