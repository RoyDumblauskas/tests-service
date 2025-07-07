use dioxus::prelude::*;

static CSS: Asset = asset!("/src/components/Service/Service.css");

#[derive(Props, PartialEq, Clone)]
struct Props {
    name: String,
}

#[component]
pub fn Service(name: String) -> Element {
    rsx! { h1 { "Service: {name}" } }
}
