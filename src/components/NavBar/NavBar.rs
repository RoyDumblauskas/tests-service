use dioxus::prelude::*;
use crate::Route;

static CSS: Asset = asset!("/assets/component-css/NavBar.css");

#[component]
pub fn NavBar() -> Element {
    rsx! { 
        document::Stylesheet { href: CSS }
        div { id: "navbar",
            Link {
                to: Route::Home { },
                "Home"
            }
            Link {
                to: Route::ServiceList { },
                "Service List"
            }
        }
        Outlet::<Route> { }
    }
}

