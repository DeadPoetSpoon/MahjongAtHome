use api::user::signup;
use dioxus::prelude::*;
#[component]
pub fn SignupForm() -> Element {
    let mut send_signup = use_action(signup);
    rsx!(
        form {
            onsubmit: move |evt: FormEvent| async move {
                evt.prevent_default();
                let values = evt.parsed_values();
                if values.is_ok() {
                    send_signup.call(values.unwrap());
                }else{
                    // TODO: show error message
                    dioxus::logger::tracing::error!("Invalid input");
                }
            },
            fieldset {
                class: "grid",
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
                label{
                    "Role",
                    select {
                        id: "role",
                        name: "role",
                        required: true,
                        option {
                            value: "user",
                            selected: true,
                            "User"
                        }
                        option {
                            value: "admin",
                            "Admin"
                        }
                        option {
                            value: "guest",
                            "Guest"
                        }
                    }
                }
            },
            button {
                r#type:"submit",
                disabled: send_signup.pending(),
                aria_busy: send_signup.pending(),
                "Signup"
            },
            if send_signup.value().is_some() {
                if send_signup.value().unwrap().is_ok() {
                    div {
                        "Signup successful"
                    }
                } else {
                    div {
                        "Signup failed {send_signup.value().unwrap():#?}"
                    }
                }
            }
        }
    )
}
