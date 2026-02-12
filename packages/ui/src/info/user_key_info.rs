use dioxus::prelude::*;
#[component]
pub fn UserKeyInfoSpan() -> Element {
    let fetch_key_info = use_resource(api::user::get_user_key_info);
    match &*fetch_key_info.read_unchecked() {
        Some(Ok(info_option)) => match info_option {
            Some(info) => {
                rsx! {
                    span {
                        style: "width:fit-content;",
                        "data-tooltip": "{info.declaration}",
                        a {
                            href: "/user_info",
                            style: "text-decoration:none;",
                            strong {
                                "{info.nickname}"
                            }
                            sup {
                                "{info.score}"
                            }
                        },
                    }
                }
            }
            None => rsx! {
                span {
                    style: "width:fit-content;",
                    "data-tooltip": "Not Found",
                    a {
                        style: "text-decoration:none;",
                        strong {
                            "NoName"
                        }
                        sup {
                            "0"
                        }
                    },
                }
            },
        },
        Some(Err(e)) => {
            rsx! {
                span {
                    style: "width:fit-content;",
                    "data-tooltip": "{e}",
                    a {
                        style: "text-decoration:none;",
                        del {
                            "Something wrong!"
                        }
                    },
                }
            }
        }
        None => rsx! { span{"Loading..."} },
    }
}
