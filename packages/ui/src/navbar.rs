use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/pico.min.css");

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        header {
            id: "navbar",
            nav {
                ul{
                   li {
                       strong {
                           "MAtH"
                       }
                   }
                }
                {children}
            }
        }
    }
}
