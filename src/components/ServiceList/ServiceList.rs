use dioxus::prelude::*;
use crate::server::{get_status, list_items_in_bucket};

static CSS: Asset = asset!("/assets/component-css/ServiceList.css");

#[component]
pub fn ServiceList() -> Element {
    let textField = use_signal(|| "".to_string());
    let resp  = use_signal(|| "".to_string()); 
    let itemList: Signal<Vec<String>> = use_signal(|| vec![]); 

    rsx! {
        document::Stylesheet { href: CSS }
        div {
            h2 { "Service List" }
            ServiceComponent { textField: textField, resp: resp, itemList: itemList}
        }
    }
}

#[component]
fn ServiceComponent(textField: Signal<String>, resp: Signal<String>, itemList: Signal<Vec<String>>) -> Element {
    rsx! {
        div { id: "container",
            h1 { "{resp.clone()}" }
            div { id: "itemList",
                {itemList.iter().map(|item| {
                    rsx! {h3 { "{item.clone()}" }}
                })}
            }
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
                    itemList.set(list_items_in_bucket()
                        .await
                        .unwrap());
                    },
               "Request"
            }
        }
    }
}
