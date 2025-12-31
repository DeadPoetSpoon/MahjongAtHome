use dioxus::prelude::*;
use ui::LoginForm;

#[component]
pub fn Login() -> Element {
    rsx! {
        LoginForm{}
    }
}
