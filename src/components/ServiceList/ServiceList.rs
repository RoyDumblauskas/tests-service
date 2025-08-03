use dioxus::prelude::*;
use crate::server::get_status;

static CSS: Asset = asset!("/assets/component-css/ServiceList.css");

#[component]
pub fn ServiceList() -> Element {
    let textField = use_signal(|| "".to_string());
    let resp = use_signal(|| "".to_string());

    rsx! {
        document::Stylesheet { href: CSS }
        div {
            h1 { "ServiceList" }
            ServiceComponent { textField: textField, resp: resp }
        }
    }
}

#[component]
fn ServiceComponent(textField: Signal<String>, resp: Signal<String>) -> Element {
    rsx! {
        div {
            h1 { "{resp.clone()}" }
            input {
                placeholder: "input request url...",
                oninput: move |event| async move {
                    let val = event.value();
                    textField.set(val);
                }
            } 
            button {
                
                onclick: move |_| async move {
                    resp.set(get_status(textField.to_string())
                        .await
                        .unwrap());
                    },
               "Request"
            }
        }
    }
}
