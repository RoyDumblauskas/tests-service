use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/component-css/ServiceList.css");

#[component]
pub fn ServiceList() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        h1 { "ServiceList" }
    }
}
