use api::user::UserLoginForm;
use dioxus::fullstack::{Form, SetCookie, SetHeader};
use dioxus::prelude::*;
const MAIN_CSS: Asset = asset!("/assets/pico.min.css");
#[component]
pub fn LoginForm() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        form {
            onsubmit: move |evt: FormEvent| async move {
                evt.prevent_default();
                let values = UserLoginForm {
                    username: evt.
                };
                // fetch_login.call(Form(values)).await;
            },
            fieldset {
                label {
                    "Username",
                    input {
                        r#type: "text",
                        id: "username",
                        name: "username",
                        placeholder: "Input User Name",
                        autocomplete: "username"
                    }
                },
                label {
                    "Password",
                    input {
                        r#type: "password",
                        id: "password",
                        name: "password",
                        placeholder: "Input password",
                        autocomplete: "password"
                    }
                }
            },
            button {
                r#type:"submit",
                value: "Login"
            }
        }

    }
}
