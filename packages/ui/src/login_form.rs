use dioxus::prelude::*;
#[component]
pub fn LoginForm() -> Element {
    let mut fetch_login = use_action(api::user::login);
    if fetch_login.value().is_some() {
        let fetch_result = fetch_login.value().unwrap();
        if fetch_result.is_ok() {
            rsx! {
                div {
                    "Result: {fetch_result:?}"
                }
                div {
                    "Go to: ",
                    a {
                        href: "/",
                        "Home"
                    },
                    " "
                    a {
                        href: "/info",
                        "Info"
                    },
                    " "
                    a {
                        href: "/chat",
                        "Chat"
                    },
                }
            }
        } else {
            let msg = fetch_result.err().unwrap().to_string();
            rsx! {
                div {
                    "Login failed: {msg}"
                }
            }
        }
    } else {
        rsx! {
            form {
                onsubmit: move |evt: FormEvent| async move {
                    evt.prevent_default();
                    let values = evt.parsed_values();
                    if values.is_ok() {
                        fetch_login.call(values.unwrap());
                    }else{
                        // TODO: show error message
                        dioxus::logger::tracing::error!("Invalid input");
                    }
                },
                fieldset {
                    label {
                        "Username",
                        input {
                            r#type: "text",
                            id: "username",
                            name: "username",
                            placeholder: "Input User Name",
                            autocomplete: "username",
                            required: true
                        }
                    },
                    label {
                        "Password",
                        input {
                            r#type: "password",
                            id: "password",
                            name: "password",
                            placeholder: "Input password",
                            autocomplete: "password",
                            required: true
                        }
                    }
                },
                button {
                    r#type:"submit",
                    disabled: fetch_login.pending(),
                    aria_busy: fetch_login.pending(),
                    "Login"
                },
            }

        }
    }
}
