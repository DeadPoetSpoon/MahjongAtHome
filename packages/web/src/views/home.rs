use dioxus::prelude::*;
use ui::info::UserKeyInfoCard;
use ui::Echo;

#[component]
pub fn Home() -> Element {
    rsx! {
        UserKeyInfoCard { }
        Echo {}
    }
}
