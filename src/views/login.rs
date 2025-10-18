use crate::models::user::UserLoginInfo;
use dioxus::prelude::*;
/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Login() -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut login = use_action(login_server);
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
                    onclick: move |_| {
                        login.call(UserLoginInfo {
                            username: username(),
                            password: password(),
                        });
                    },
                    "Login"
                }
            }
        }
    }
}

#[cfg(feature = "server")]
use {
    crate::entities::user,
    dioxus::fullstack::ServerFnError,
    dioxus::logger::tracing,
    sea_orm::ActiveValue::Set,
    sea_orm::{ActiveModelTrait, Database},
};

#[post("/api/login",ext: crate::AppServerStateExtension)]
async fn login_server(info: UserLoginInfo) -> ServerFnResult<()> {
    tracing::debug!("User login: {:?}", info.username);
    let user = user::ActiveModel {
        username: Set(info.username),
        password: Set(info.password),
        ..Default::default()
    };
    user.insert(&ext.db)
        .await
        .map_err(|err| ServerFnError::ServerError {
            message: format!("DB Error {err:?}"),
            code: 500u16,
            details: None,
        })?;
    tracing::debug!("Login finish");
    Ok(())
}
