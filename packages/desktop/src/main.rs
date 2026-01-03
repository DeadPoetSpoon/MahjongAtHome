use dioxus::prelude::*;

use ui::Navbar;
use views::{Blog, Home};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(DesktopNavbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

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
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}

/// A desktop-specific Router around the shared `Navbar` component
/// which allows us to use the desktop-specific `Route` enum.
#[component]
fn DesktopNavbar() -> Element {
    rsx! {
        Navbar {
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}
