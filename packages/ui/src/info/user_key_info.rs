use dioxus::prelude::*;
#[component]
pub fn UserKeyInfoCard() -> Element {
    let fetch_key_info = use_resource(api::user::get_user_key_info);
    match &*fetch_key_info.read_unchecked() {
        Some(Ok(info_option)) => match info_option {
            Some(info) => {
                rsx! { p{"{info.nickname}"}  }
            }
            None => rsx! { p{"Found no user info..."} },
        },
        Some(Err(e)) => {
            rsx! { p{"Errors: {e}"} }
        }
        None => rsx! { p{"Loading..."} },
    }
}
