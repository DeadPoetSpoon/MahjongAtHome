use dioxus::prelude::*;
use ui::info::UserInfoForm;

#[component]
pub fn UserInfo() -> Element {
    rsx! {
        article {
            UserInfoForm { }
        }
    }
}
