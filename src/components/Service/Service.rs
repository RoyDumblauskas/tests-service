use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/component-css/Service.css");

#[derive(Props, PartialEq, Clone)]
struct Props {
    name: String,
}

#[component]
pub fn Service(name: String) -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        h1 { "Service: {name}" }
    }
}
