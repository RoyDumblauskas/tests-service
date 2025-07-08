use dioxus::prelude::*;
use crate::Route;

static CSS: Asset = asset!("/assets/component-css/NavBar.css");

#[component]
pub fn NavBar() -> Element {
    let mut pageSelected = use_signal(|| "Home".to_string()); 

    rsx! { 
        document::Stylesheet { href: CSS }
        div { id: "navbar",
            Link { 
                id: if pageSelected.to_string() == ("Home".to_string()) {"selectedRoute"} else {"unselectedRoute"},
                to: Route::Home { },
                onclick: move |_| pageSelected.set("Home".to_string()),
                "Home"
            }
            Link {
                id: if pageSelected.to_string() == ("ServiceList".to_string()) {"selectedRoute"} else {"unselectedRoute"},
                to: Route::ServiceList { },
                onclick: move |_| pageSelected.set("ServiceList".to_string()),
                "Service List"
            }
        }
        Outlet::<Route> { }
    }
}

