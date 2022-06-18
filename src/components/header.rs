use dioxus::{prelude::*, router::*};

#[derive(PartialEq, Props)]
pub struct Props<'a> {
    name: &'a str,
    pseudonym: &'a str,
    headline: &'a str,
}

pub fn component<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    let segment = use_route(&cx).last_segment().unwrap();

    let link_class = "transition hover:opacity-50 focus:opacity-50 text-2xl mr-2 ".to_owned();
    let mut se_class = link_class.clone() + "text-moss";
    let mut p_class = link_class.clone() + "text-ice";
    let mut c_class = link_class.clone() + "text-sponge";

    if segment == "" {
        se_class = se_class.to_string() + " underline";
    }

    if segment == "photography" {
        p_class = p_class.to_string() + " underline";
    }

    cx.render(rsx! {
        header {
            class: "w-full flex justify-center text-white flex flex-col mb-8",
            div {
                class: "mb-3",
                h1 {
                    class: "text-slate text-2xl font-semibold",
                    "{cx.props.name}"
                    span {
                        class: "text-slate-dim",
                        " /{cx.props.pseudonym}"
                    }
                }
            }
            div {
                h2 {
                    class: "text-white text-2xl",
                    "{cx.props.headline}"
                }
            }
            div {
                class: "flex",
                Link {
                    to: "/"
                    h1 {
                        class: "{se_class}",
                        "Programmer."
                    }
                }

                Link {
                    to: "/photography"
                    h2 {
                        class: "{p_class}",
                     "Photographer."
                    }
                }

                Link {
                    to: "/cars"
                    h2 {
                        class: "{c_class}",
                     "Likes Cars."
                    }
                }
            }
        }
    })
}
