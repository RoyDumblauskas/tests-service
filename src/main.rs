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
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    rsx! {
        document::Stylesheet { href: CSS }
        Router::<Route> { }
    }
}

