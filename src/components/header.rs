use dioxus::{prelude::*, router::*};

#[derive(PartialEq, Props)]
pub struct Props<'a> {
    name: &'a str,
    pseudonym: &'a str,
    headline: &'a str,
    social: &'a crate::config::SocialConfig,
}

pub fn component<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    let segment = use_route(&cx).last_segment().unwrap();

    let link_class = "text-xl lg:text-2xl mr-2 cursor-default ".to_owned();
    let mut se_class = link_class.clone()
        + "text-moss transition hover:opacity-50 focus:opacity-50 cursor-pointer    ";
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
            class: "w-full text-white flex mb-8 relative",
            div {
                class: "flex flex-col justify-center",
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
                class: "flex flex-wrap",
                Link {
                    to: "/"
                    h1 {
                        class: "{se_class}",
                        "Programmer."
                    }
                }

                Link {
                    to: "/"
                    h2 {
                        class: "{p_class}",
                     "Photographer."
                    }
                }

                Link {
                    to: "/"
                    h2 {
                        class: "{c_class}",
                     "Likes Cars."
                    }
                }
            }

            }
            div {
                class: "flex-1 absolute w-full",
                ul {
                    class: "flex-col flex lg:flex-row justify-end text-right w-full",
                    vec!(("Twitter", "twitter.com/", cx.props.social.twitter), ("Linkedin", "linkedin.com/in/", cx.props.social.linkedin), ("GitHub", "github.com/", cx.props.social.github))
                    .iter()
                    .map(|(site, domain, username)| rsx! { li { a  { class: "text-slate underline text-xs transition hover:text-moss ml-2", href: "https://{domain}{username}", "{site}" } } })
                }
            }
        }
    })
}
