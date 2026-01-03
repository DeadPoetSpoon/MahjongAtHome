use dioxus::prelude::*;
use dioxus_fullstack::{FullstackContext, StatusCode};
use ui::Navbar;
use views::{Home, Login, Manage};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(WebMainLayout)]
    #[route("/")]
    Home {},
    #[route("/login")]
    Login {},
    #[route("/manage")]
    Manage {},
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/pico.min.css");

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use dioxus::logger::tracing;
        use dioxus::server::axum::Extension;

        // init app state
        let mut state = api::AppServerState::default();
        let config_path = "math.toml";
        let config = if std::fs::exists(config_path)? {
            tracing::debug!("math.toml found, read config from file");
            match api::AppInitServerConfig::from_file(config_path) {
                Ok(c) => c,
                Err(err) => {
                    tracing::error!("read math.toml error: {:?}\nuse default config", err);
                    Default::default()
                }
            }
        } else {
            tracing::debug!("no math.toml found, use default config");
            Default::default()
        };

        match state.init(config).await {
            Ok(_) => {}
            Err(err) => {
                panic!("init app state error: {:?}", err);
            }
        };
        let router = dioxus::server::router(App).layer(Extension(state));
        Ok(router)
    });
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebMainLayout() -> Element {
    rsx! {

        Navbar {
            ul{
                li{
                    Link {
                        to: Route::Home {},
                        "Home"
                    }
                }
                li{
                    details {
                        class: "dropdown",
                        summary {
                            "M"
                        }
                        ul {
                            dir: "rtl",
                            li {
                                Link {
                                    to: Route::Login {},
                                    "Login"
                                }
                            }
                            li {
                                Link {
                                    to: Route::Manage {},
                                    "Manage"
                                }
                            }
                        }
                    }
                }
            }
        }
        ErrorBoundary{
            handle_error: move |err: ErrorContext| {
                let http_error = FullstackContext::commit_error_status(err.error().unwrap());
                match http_error.status {
                    StatusCode::NOT_FOUND => rsx! { div { "404 - Page not found" } },
                    StatusCode::UNAUTHORIZED => rsx! { div { "401 - Unauthorized" } },
                    StatusCode::INTERNAL_SERVER_ERROR => rsx! { div { "500 - Internal Server Error" } },
                    _ => rsx! { div { "An unknown error occurred" } },
                }
            },
            main {
                class:"container",
                Outlet::<Route> {}
            }
        }

    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattempted to navigate to: {route:?}" }
    }
}
