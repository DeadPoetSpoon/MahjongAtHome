use crate::models::user::UserLoginInfo;
use crate::server;
use dioxus::prelude::*;
#[component]
pub fn Dashboard() -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    // let server_init = use_resource(move || async move { server::has_init_server().await });
    let server_init = use_server_future(server::has_init_server)?;
    rsx!(
        if let Some(server_has_init) = &*server_init.read() {
            if let Ok(server_has_init) = server_has_init {
                div {
                    class: "accordion",
                    id: "dashboard",
                    div {
                        class: "accordion-item",
                        h2 {
                            class: "accordion-header",
                            button {
                                class: "accordion-button",
                                r#type: "button",
                                "data-bs-toggle": "collapse",
                                "data-bs-target": "#collapse-server-status",
                                "aria-expanded": "false",
                                "aria-controls": "collapse-server-status",
                                "Server Status"
                            }
                        }
                        div {
                            id: "collapse-server-status",
                            class: "accordion-collapse collapse show",
                            "data-bs-parent": "#dashboard",
                            div{
                                class: "accordion-body",
                                if *server_has_init {
                                    "Server has been initialized"
                                }else{
                                    button {
                                        class: "btn btn-primary w-100 py-2",
                                        r#type: "button",
                                        onclick: move |_| async move {
                                            let _ = server::init_server().await;
                                        },
                                        "Init Server",
                                    }
                                }
                            }
                        }
                    },

                    if *server_has_init {
                        div {
                            class: "accordion-item",
                            h2 {
                                class: "accordion-header",
                                button {
                                    class: "accordion-button",
                                    r#type: "button",
                                    "data-bs-toggle": "collapse",
                                    "data-bs-target": "#collapse-user-management",
                                    "aria-expanded": "false",
                                    "aria-controls": "collapse-user-management",
                                    "User Management"
                                }
                            }
                            div {
                                id: "collapse-user-management",
                                class: "accordion-collapse collapse",
                                "data-bs-parent": "#dashboard",
                                div{
                                    class: "accordion-body",
                                    div {
                                        class: "row",
                                        div {
                                            class: "col-md-5 align-content-center",
                                            input {
                                                r#type: "text",
                                                class: "form-control",
                                                placeholder: "Username",
                                                aria_label: "Username",
                                                oninput: move |e| {
                                                    username.set(e.value());
                                                }
                                            }
                                        },
                                        div {
                                            class: "col-md-5 align-content-center",
                                            input {
                                                r#type: "text",
                                                class: "form-control",
                                                placeholder: "Password",
                                                aria_label: "Password",
                                                oninput: move |e| {
                                                    password.set(e.value());
                                                }
                                            }
                                        },
                                        div {
                                            class: "col-md-2",
                                            button {
                                                class: "btn btn-primary w-100 py-2 my-4",
                                                r#type: "button",
                                                onclick: move |_| async move {
                                                    let user_info = super::USERINFO.read();
                                                    if user_info.is_some() {
                                                        let token = &user_info.as_ref().unwrap().token;
                                                        let _ = server::signup_server(UserLoginInfo { username:username(), password:password(),token:Some(token.clone()) }).await;

                                                    }

                                                },
                                                "Signup"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }else{
                "Error: {server_has_init:?}"
            }
        }else{
            "Loading..."
        }

    )
}
