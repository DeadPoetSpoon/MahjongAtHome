use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    let navigator = use_navigator();
    navigator.push(Route::Home {});
    rsx! {
        div {
            h1 { "Page Not Found" }
            p { "The page {route:?} you are looking for does not exist." }
        }
    }
}
