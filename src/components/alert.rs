use dioxus::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub enum AlertType {
    Error,
    Warning,
    Success,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub struct AlertInfo {
    pub info_type: AlertType,
    pub message: String,
}

#[component]
fn one_alert(info: AlertInfo) -> Element {
    let div_class = match info.info_type {
        AlertType::Error => "alert-danger",
        AlertType::Warning => "alert-warning",
        AlertType::Success => "alert-info",
    };
    let div_class = format!(
        "{} {}",
        div_class, "alert m-2 position-absolute bottom-0 end-0 alert-dismissible fade show"
    );
    rsx! {
        div {
            key: info.message,
            class: div_class,
            role: "alert",
            "{info.message}"
            button {
                class: "btn-close",
                r#type: "button",
                onclick: move |_| {
                    let mut infos = super::ALERTS.read().clone();
                    infos.retain(|i| i.message != info.message);
                    *super::ALERTS.write() = infos;
                }
            }
        }
    }
}

#[component]
pub fn alert_view() -> Element {
    let infos = super::ALERTS.read();
    if infos.len() == 0 {
        return rsx! {};
    }
    rsx! {
        div {
            class: "position-fixed bottom-0 end-0",
            style: "z-index: 1000;width:300px;height:200px;",
            ol {
                class: "list-group list-group-numbered",
                for info in infos.iter() {
                    li {
                        class: "list-group-item d-flex justify-content-between align-items-center",
                        key: info.message,
                        one_alert{info:info.clone()}
                    }
                }
            }
        }
    }
}
