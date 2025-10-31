//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.

mod hero;
use dioxus::signals::{GlobalSignal, Signal};
pub use hero::hero_view;

mod echo;
pub use echo::echo_view;

mod alert;
pub use alert::*;

pub static ALERTS: GlobalSignal<Vec<AlertInfo>> = Signal::global(|| Vec::new());
