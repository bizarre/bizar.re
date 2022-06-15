use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct Props<'a> {
    name: &'a str,
    pseudonym: &'a str,
    headline: &'a str,
}

pub fn component<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    cx.render(rsx! {
        header {
            class: "w-full flex justify-center text-white flex flex-col",
            div {
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
        }
    })
}
