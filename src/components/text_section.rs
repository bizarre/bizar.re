use dioxus::{prelude::*, router::*};

#[derive(PartialEq, Props)]
pub struct Props<'a> {
    title: &'a str,
    subtitle: &'a str,
    text: &'a str,
    span_class: Option<&'a str>,
}

pub fn component<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    let span_class = cx.props.span_class.unwrap_or("");

    cx.render(rsx! {
        section {
            header {
                class: "mb-2",
                h3 {
                    class: "text-xl text-dim",
                    "{cx.props.title} "
                    span {
                        class: "{span_class}",
                        "{cx.props.subtitle}"
                    }
                }
            }
            article {
                p {
                    class: "text-slate",
                    "{cx.props.text}"
                }
            }
        }
    })
}
