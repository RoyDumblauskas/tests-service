use dioxus::prelude::*;
use crate::Route;

static CSS: Asset = asset!("/assets/component-css/NavBar.css");

#[component]
pub fn NavBar() -> Element {
    let page = use_route::<Route>().to_string();
    let mut pageSelected = use_signal(|| page.clone()); 

    rsx! { 
        document::Stylesheet { href: CSS }
        div { id: "navbar",
            Link { 
                id: if pageSelected.to_string() == ("/".to_string()) {"selectedRoute"} else {"unselectedRoute"},
                to: Route::Home { },
                onclick: move |_| pageSelected.set("/".to_string()),
                "Home"
            }
            Link {
                id: if pageSelected.to_string() == ("/services".to_string()) {"selectedRoute"} else {"unselectedRoute"},
                to: Route::ServiceList { },
                onclick: move |_| pageSelected.set("/services".to_string()),
                "Service List"
            }
        }
        Outlet::<Route> { }
    }
}

