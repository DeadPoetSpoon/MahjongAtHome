use crate::components::{echo_view, hero_view, AlertInfo, AlertType};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let mut infos = crate::components::ALERTS.read().clone();
    let mut count = use_signal(|| 0);
    rsx! {
        hero_view {}
        echo_view {}
        button {
            r#type: "button",
            class: "btn btn-primary",
            onclick: move |_| {
                let alert_info = AlertInfo {
                    info_type: AlertType::Success,
                    message: format!("Alert added {}",count()),
                };
                infos.push(alert_info);
                *crate::components::ALERTS.write() = infos.clone();
                count.set(count() + 1);
            },
            "Add Alert"
        }
    }
}
