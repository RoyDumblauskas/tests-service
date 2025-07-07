use dioxus::prelude::*;

static CSS: Asset = asset!("/src/components/ServiceList/ServiceList.css");

#[component]
pub fn ServiceList() -> Element {
    rsx! { h1 { "ServiceList" } }
}
