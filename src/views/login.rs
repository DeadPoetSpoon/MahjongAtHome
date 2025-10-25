use crate::models::user::UserLoginInfo;
use crate::server;
use dioxus::prelude::*;
/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Login() -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    // let mut login = use_server_future(login_server);
    rsx! {
        main {
            class:"d-flex vh-100 w-100 align-items-center bg-body-tertiary form-signin m-auto justify-content-center",
            form {
                h1 {
                    class:"h3 mb-3 fw-normal mb-4",
                    "Mahjong At Home"
                }
                div {
                    class: "form-floating",
                    input {
                        class: "form-control",
                        type: "email",
                        id: "floatingUsername",
                        placeholder: "user@math.com",
                        oninput: move |e| {
                            username.set(e.value());
                        }
                    }
                    label {
                        for: "floatingUsername",
                        "Username"
                    }
                }
                div {
                    class: "form-floating",
                    input {
                        class: "form-control",
                        type: "password",
                        id: "floatingPassword",
                        placeholder: "Password",
                        oninput: move |e| {
                            password.set(e.value());
                        }
                    }
                    label {
                        for: "floatingPassword",
                        "Password"
                    }
                }
                button {
                    class: "btn btn-primary w-100 py-2 my-4",
                    type: "button",
                    onclick: move |_| async move {
                        let _ = server::login_server(UserLoginInfo { username:username(), password:password() }).await;
                    },
                    "Login"
                }
                button {
                    class: "btn btn-secondary w-100 py-2",
                    type: "button",
                    onclick: move |_| async move {
                        let _ = server::init_server().await;
                    },
                    "Init Server",
                }
            }
        }
    }
}
