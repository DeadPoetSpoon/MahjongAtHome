use dioxus::prelude::*;
use dioxus_html::colgroup::span;
#[component]
pub fn UserInfoForm() -> Element {
    let fetch_info = use_resource(api::user::get_user_info);
    let info = fetch_info.read_unchecked();
    if info.is_none() {
        return rsx! {
            article {
                aria_busy: "true",
            }
        };
    }
    let info = info.as_ref().unwrap();
    if info.is_err() {
        let err = info.as_ref().err().unwrap();
        return rsx! {
            article {
                "data-tooltip":"{err}",
                "Something wrong!"
            }
        };
    }
    let info = info.as_ref().unwrap();
    if info.is_none() {
        return rsx! {
            article {
                "No Found"
            }
        };
    }
    let info = info.as_ref().unwrap();
    rsx! {
        h3 {
            "User Info"
        }
        p {
            u {"{info.nickname}"}sup { "{info.score}" }
        }
        p{
            if info.gender < 0 {
                span{"male"}
            }else if info.gender > 0 {
                span {"female"}
            }else{
                span {"UnKonw"}
            }
            sup { u {"{info.gender}"} }span{", "}
            u {"{info.age}"}span {" years old, "}
            span { "live at " }u { "{info.location}" }
        }
        p {
            span { "TA's fight declaration is: " } u { "{info.declaration}" }
        }
        div {
            class: "grid",
            div {
                h4 { "MBTI" }
                div {
                    display:"flex",
                    align_items: "center",
                    strong {
                        width:"30px",
                        text_align:"center",
                        "E"
                    }
                    progress {
                        margin_bottom:"0px",
                        max:"256",
                        value:"{info.mbti_ei + 128}"
                    }
                    strong {
                        width:"30px",
                        text_align:"center",
                        "I"
                    }
                }
                div {
                    display:"flex",
                    align_items: "center",
                    strong {
                        width:"30px",
                        text_align:"center",
                        "N"
                    }
                    progress {
                        margin_bottom:"0px",
                        max:"256",
                        value:"{info.mbti_ns + 128}"
                    }
                    strong {
                        width:"30px",
                        text_align:"center",
                        "S"
                    }
                }
                div {
                    display:"flex",
                    align_items: "center",
                    strong {
                        width:"30px",
                        text_align:"center",
                        "F"
                    }
                    progress {
                        margin_bottom:"0px",
                        max:"256",
                        value:"{info.mbti_ft + 128}"
                    }
                    strong {
                        width:"30px",
                        text_align:"center",
                        "T"
                    }
                }
                div {
                    display:"flex",
                    align_items: "center",
                    strong {
                        width:"30px",
                        text_align:"center",
                        "J"
                    }
                    progress {
                        margin_bottom:"0px",
                        max:"256",
                        value:"{info.mbti_jp + 128}"
                    }
                    strong {
                        width:"30px",
                        text_align:"center",
                        "P"
                    }
                }
            }
            div {
                h4 { "Fight Info" }
                div {
                    display:"flex",
                    justify_content: "center",
                    strong { "Speed" }
                }
                div {
                    display:"flex",
                    align_items: "center",
                    strong {
                        width:"110px",
                        text_align:"center",
                        "Sloth"
                    }
                    progress {
                        margin_bottom:"0px",
                        max:"256",
                        value:"{info.speed + 128}"
                    }
                    strong {
                        width:"110px",
                        text_align:"Cheetah",
                        "Cheetah"
                    }
                }

                div {
                    display:"flex",
                    justify_content: "center",
                    strong { "Speek" }
                }
                div {
                    display:"flex",
                    align_items: "center",
                    strong {
                        width:"110px",
                        text_align:"center",
                        "Jellyfish"
                    }
                    progress {
                        margin_bottom:"0px",
                        max:"256",
                        value:"{info.speek + 128}"
                    }
                    strong {
                        width:"110px",
                        text_align:"center",
                        "Cicala"
                    }
                }
            }
        }
    }
}
