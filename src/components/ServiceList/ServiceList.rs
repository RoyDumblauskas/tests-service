use dioxus::prelude::*;
use crate::server::{get_item_in_bucket, get_status, list_items_in_bucket, put_item_in_bucket, delete_item_in_bucket};

static CSS: Asset = asset!("/assets/component-css/ServiceList.css");

#[component]
pub fn ServiceList() -> Element {
    let textField = use_signal(|| "".to_string());
    let fileName = use_signal(|| "".to_string());
    let itemKey = use_signal(|| "".to_string()); 
    let deleteItemKey = use_signal(|| "".to_string()); 
    let resp  = use_signal(|| "".to_string()); 
    let itemList: Signal<Vec<String>> = use_signal(|| vec![]); 

    rsx! {
        document::Stylesheet { href: CSS }
        div {
            h2 { "Service List" }
            ServiceComponent { 
                textField: textField, 
                fileName: fileName, 
                itemKey: itemKey, 
                deleteItemKey: deleteItemKey,
                resp: resp, 
                itemList: itemList
            }
        }
    }
}

#[component]
fn ServiceComponent(
    textField: Signal<String>, 
    fileName: Signal<String>, 
    itemKey: Signal<String>, 
    deleteItemKey: Signal<String>,
    resp: Signal<String>, 
    itemList: Signal<Vec<String>>) 
    -> Element {
    rsx! {
        div { id: "container",
            h1 { "{resp.clone()}" }
            div { id: "itemList",
                {itemList.iter().map(|item| {
                    rsx! {h3 { "{item.clone()}" }}
                })}
            }
            button {
                onclick: move |_| async move {
                    itemList.set(list_items_in_bucket()
                        .await
                        .unwrap());

                },
                "List Items"
            }
            div { id: "putObject",
                input {
                    placeholder: "Name of file",
                    value: fileName.clone().to_string(),
                    oninput: move |event| async move {
                        fileName.set(event.value());
                    }
                }
                input {
                    placeholder: "Write string to object",
                    value: textField.clone().to_string(),
                    oninput: move |event| async move {
                        textField.set(event.value());
                    }
                } 
                button {
                    onclick: move |_| async move {
                        put_item_in_bucket(fileName.clone().to_string(), textField.clone().to_string())
                            .await
                            .unwrap()
                            .to_string();
                        fileName.set("".to_string());
                        textField.set("".to_string());
                    },
                    "Put Item"
                }
            }
            div { id: "getItem",
                input {
                    placeholder: "Item key",
                    value: itemKey.clone().to_string(),
                    oninput: move |event| async move {
                        itemKey.set(event.value())
                    }
                }
                button {
                    onclick: move |_| async move {
                        resp.set(get_item_in_bucket(itemKey.clone().to_string())
                            .await
                            .unwrap()
                            .to_string());
                        itemKey.set("".to_string());
                    },
                    "Get Item"
                }
            }
            div { id: "deleteItem",
                input {
                    placeholder: "Item key",
                    value: deleteItemKey.clone().to_string(),
                    oninput: move |event| async move {
                        deleteItemKey.set(event.value())
                    }
                }
                button {
                    onclick: move |_| async move {
                        delete_item_in_bucket(deleteItemKey.clone().to_string())
                            .await
                            .unwrap()
                            .to_string();
                        deleteItemKey.set("".to_string());
                    },
                    "Delete Item"
                }
            }
        }
    }
}
