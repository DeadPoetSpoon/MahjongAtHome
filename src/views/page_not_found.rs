use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            "Page Not Found"
        }
        Link {
            class: "nav-link",
            to: Route::Login {},
            "Login"
        }
    }
}
