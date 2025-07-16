use dioxus::prelude::*;
use crate::server::*;

static CSS: Asset = asset!("/assets/component-css/ServiceList.css");

#[component]
pub fn ServiceList() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        div {
            h1 { "ServiceList" }
            ServiceComponent { statusURL: "https://google.com"}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ServCompProps {
    statusURL: String,
}

#[component]
fn ServiceComponent(statusURL: String) -> Element {
    
    rsx! {
       div { "{statusURL}" } 
    }
}
