use dioxus::prelude::*;
use ui::LoginForm;

#[component]
pub fn Login() -> Element {
    rsx! {
        div {
            class: "container",
            justify_items: "center",
            h3 {
                "Riichi " u{"M"} "ahjong" u { "At" } " " u { "H" } "ome"
            },
            h3 {

            },
        }
        div{
            class:"grid",
            div {

            },
            div {
                LoginForm{}
            },
            div {

            },
        }
    }
}
