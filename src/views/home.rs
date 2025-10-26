use crate::{
    components::{echo_view, hero_view},
    Route,
};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        hero_view {}
        echo_view {}
        Link {
            class: "nav-link",
            to: Route::Home {},
            "Home"
        }
    }
}
