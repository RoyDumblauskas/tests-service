use dioxus::prelude::*;

// Use components
mod components;

use crate::components::*;

static CSS: Asset = asset!("/assets/main.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home { },
    #[route("/services")]
    ServiceList { },
    #[route("/service/:name")]
    Service { name: String },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let img = use_signal(|| "example.jpg".to_string());

    rsx! {
        document::Stylesheet { href: CSS }
        NavBar { }
        div { id: "app",
            h1 { "Blog Prototype" }
            SearchBar { img: img }
            img { id: "im", src: "https://imgs.roypository.com/dev/{img}" }
        }
    }
}

#[component]
fn SearchBar(mut img: Signal<String>) -> Element {
    let mut searchBarText = use_signal(|| "example.jpg".to_string());
    
    rsx! {
        div { id: "searchbar",
            input {
                id: "inp",
                value: "{searchBarText}",
                oninput: move |event| searchBarText.set(event.value())
            }
            button {
                id: "searchbutton",
                onclick: move |_| img.set(searchBarText.to_string()),
                "Search",
            }
        }
    }
}
