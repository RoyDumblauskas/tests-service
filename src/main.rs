use dioxus::prelude::*;
static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    // import stylesheets

    let img = use_signal(|| "example.jpg".to_string() );

    rsx! {
        document::Stylesheet { href: CSS }
        div { id: "app",
            h1 { "Blog Prototype" }
            div { id: "content",
                SearchBar { img: img }
                img { id: "im", src: "https://imgs.roypository.com/dev/{img}" }
            }
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
