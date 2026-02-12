use dioxus::prelude::*;
use ui::info::UserKeyInfoSpan;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            h3 {
                "Hello, "
                UserKeyInfoSpan { }
            }
        }

        // Echo {}
    }
}
