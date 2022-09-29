use crate::components::Tooltip;
use dioxus::{prelude::*, router::*};

#[derive(PartialEq, Props)]
pub struct Props<'a> {
    name: &'a str,
    pseudonym: &'a str,
    headline: &'a str,
    social: &'a crate::config::SocialConfig,
}

pub fn component<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    let mut is_info_tooltip_open = use_state(&cx, || false);
    let segment = use_route(&cx).last_segment().unwrap();

    let link_class = "text-xl mr-2 cursor-default ".to_owned();
    let mut se_class = link_class.clone()
        + "text-lavender transition hover:opacity-50 focus:opacity-50 cursor-pointer";
    let mut p_class =
        link_class.clone() + "text-ice transition hover:opacity-50 focus:opacity-50 cursor-pointer";
    let mut c_class = link_class.clone() + "text-sponge";

    if segment == "" {
        se_class = se_class.to_string() + " underline";
    }

    if segment == "photography" {
        p_class = p_class.to_string() + " underline";
    }

    cx.render(rsx! {
        header {
            class: "w-full text-white flex mb-6 relative",
            div {
                class: "flex flex-col justify-center",
                div {
                    class: "mb-3 flex items-center",
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
                    class: "text-white text-xl",
                    "{cx.props.headline}"
                }
            }
            div {
                class: "flex flex-wrap",
                Link {
                    to: "/"
                    h1 {
                        class: "{se_class}",
                        "nerd."
                    }
                }

                Link {
                    to: "/photography"
                    h2 {
                        class: "{p_class}",
                     "photog."
                    }
                }

                Link {
                    to: "/"
                    h2 {
                        class: "{c_class}",
                     "hoonigan."
                    }
                }
            }

            }
            div {
                class: "flex-1 absolute w-full flex items-center",
                ul {
                    class: "flex-col flex lg:flex-row justify-end text-right w-full",
                    vec!(("shit posts", "twitter.com/", cx.props.social.twitter), ("linkedin", "linkedin.com/in/", cx.props.social.linkedin), ("github", "github.com/", cx.props.social.github))
                    .iter()
                    .map(|(site, domain, username)| rsx! { li { a  { class: "text-slate underline text-xs transition hover:text-lavender ml-2", href: "https://{domain}{username}", "{site}" } } })
                }
                div {
                    class: "hidden lg:flex h-3 w-3 top-0 ml-4 mt-1 bg-lavender rounded-full items-center justify-center text-black text-xs relative",
                    onmouseover: move |_| { is_info_tooltip_open.set(true)},
                    onmouseout: move |_| { is_info_tooltip_open.set(false)},
                    "!"
                    div {
                        class: "absolute w-full h-full bg-lavender rounded-full animate-ping cursor-pointer"
                    }

                    Tooltip {
                        is_open: *is_info_tooltip_open.get(),
                        pointer_events: true,
                        class: "ml-0 px-6 mt-12 bg-black z-40",
                        rsx!{
                            p {
                                class: "text-lavender text-xs",
                                "This site is under pretty heavy development. Expect some shit to break and/or be jank af. "
                                br{}br{}
                                "Written in Rust with "
                                a {
                                     href: "http://dioxuslabs.com",
                                     target: "_blank",
                                     class: "underline",
                                    "Dioxus"
                                }
                                " and built w/ "
                                a {
                                    href: "https://trunkrs.dev",
                                    target: "_blank",
                                    class: "underline",
                                    "Trunk"
                                }
                                ". "
                                br{}br{}
                                a {
                                    href: "https://github.com/bizarre/bizar.re",
                                    target: "_blank",
                                    class: "underline",
                                    "Source code available on GitHub."
                                }

                            }
                        }
                    }
                }
            }
        }
    })
}
