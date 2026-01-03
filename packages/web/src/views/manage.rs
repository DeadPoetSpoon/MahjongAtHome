use dioxus::prelude::*;
use ui::manage::*;

#[component]
pub fn Manage() -> Element {
    rsx! {
        article {
            header {
                h3{
                    "User"
                }
            }
            details {
                summary {
                    "User Signup"
                },
                user::SignupForm{}
            }

        }
    }
}
