// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use views::{Blog, Home, Login, Navbar};

/// Define a components module that contains all shared components for our app.
mod components;
mod models;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
///
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Login{},
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
    #[layout(Navbar)]
        // The route attribute defines the URL pattern that a specific route matches. If that pattern matches the URL,
        // the component for that route will be rendered. The component name that is rendered defaults to the variant name.
        #[route("/home")]
        Home {},
        // The route attribute can include dynamic parameters that implement [`std::str::FromStr`] and [`std::fmt::Display`] with the `:` syntax.
        // In this case, id will match any integer like `/blog/123` or `/blog/-456`.
        #[route("/blog/:id")]
        // Fields of the route variant will be passed to the component as props. In this case, the blog component must accept
        // an `id` prop of type `i32`.
        Blog { id: i32 },
}

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    // On the client we just launch the app as normal.
    // #[cfg(not(feature = "server"))]
    dioxus::launch(app);

    // On the server, we can use `dioxus::serve` and `.serve_dioxus_application` to serve our app with routing.
    // The `dioxus::server::router` function creates a new axum Router with the necessary routes to serve the Dioxus app.
    // #[cfg(feature = "server")]
    // dioxus::serve(|| async move {
    //     use dioxus::server::axum::Extension;
    //     use migration::{Migrator, MigratorTrait};
    //     dioxus::logger::tracing::debug!("start connect to db");
    //     let db =
    //         Database::connect("postgres://spoon:mahjong_at_home@localhost:5432/mahjong_at_home")
    //             .await
    //             .expect("Failed to connect to database");

    //     dioxus::logger::tracing::debug!("finish connect to db");
    //     dioxus::logger::tracing::debug!("start migrator up");
    //     Migrator::up(&db, None)
    //         .await
    //         .expect("Failed to migrator up");
    //     dioxus::logger::tracing::debug!("finish migrator up");
    //     Ok(dioxus::server::router(app).layer(Extension(AppServerState { db })))
    // });
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn app() -> Element {
    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }

        document::Stylesheet { href: "https://cdn.jsdelivr.net/npm/bootstrap@5.3.8/dist/css/bootstrap.min.css" }
        document::Script {src: "https://cdn.jsdelivr.net/npm/@popperjs/core@2.11.8/dist/umd/popper.min.js"}
        document::Script {src: "https://cdn.jsdelivr.net/npm/bootstrap@5.3.8/dist/js/bootstrap.bundle.min.js"}

        // The router component renders the route enum we defined above. It will handle synchronization of the URL and render
        // the layouts and components for the active route.
        Router::<Route> {}
    }
}

#[cfg(feature = "server")]
mod entities;
#[cfg(feature = "server")]
use {
    dioxus::fullstack::Lazy,
    sea_orm::{Database, DatabaseConnection},
};
#[cfg(feature = "server")]
#[derive(Clone, Debug, Default)]
pub struct AppServerState {
    db: DatabaseConnection,
}

// #[cfg(feature = "server")]
// type AppServerStateExtension = axum::Extension<AppServerState>;

// #[cfg(feature = "server")]
// static APPSTATE: LazyLock<tokio::sync::RwLock<AppServerState>> = std::sync::LazyLock::new(|| {
//     let rt = tokio::runtime::Builder::new_current_thread()
//         .build()
//         .unwrap();
//     let db = rt.block_on(async {
//         use migration::{Migrator, MigratorTrait};
//         let db =
//             Database::connect("postgres://spoon:mahjong_at_home@localhost:5432/mahjong_at_home")
//                 .await
//                 .expect("Failed to connect to database");

//         dioxus::logger::tracing::debug!("finish connect to db");
//         dioxus::logger::tracing::debug!("start migrator up");
//         Migrator::up(&db, None)
//             .await
//             .expect("Failed to migrator up");
//         dioxus::logger::tracing::debug!("finish migrator up");
//         db
//     });
//     tokio::sync::RwLock::new(AppServerState { db })
// });

#[cfg(feature = "server")]
static APPSTATE: Lazy<AppServerState> = Lazy::new(|| async move {
    let (db,) = tokio::join!(async {
        use migration::{Migrator, MigratorTrait};
        let db =
            Database::connect("postgres://spoon:mahjong_at_home@localhost:5432/mahjong_at_home")
                .await
                .expect("Failed to connect to database");

        dioxus::logger::tracing::debug!("finish connect to db");
        dioxus::logger::tracing::debug!("start migrator up");
        Migrator::up(&db, None)
            .await
            .expect("Failed to migrator up");
        dioxus::logger::tracing::debug!("finish migrator up");
        db
    });
    dioxus::Ok(AppServerState { db })
});
