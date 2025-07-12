use dioxus::prelude::*;

// Use components
mod components;

use crate::components::*;

static CSS: Asset = asset!("/assets/main.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home { },
    #[route("/services")]
    ServiceList { },
    #[route("/service/:name")]
    Service { name: String },
}

fn main() {
    #[cfg(feature = "web")]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                launch_server(App).await;
            });
    }
}

#[component]
fn App() -> Element {

    rsx! {
        document::Stylesheet { href: CSS }
        div { id: "control",
            div{ id: "wrap",
                Router::<Route> { }
            }
        }
    }
}
