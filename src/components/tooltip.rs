use dioxus::{prelude::*, router::*};

#[derive(Props)]
pub struct Props<'a> {
    children: Element<'a>,
    class: Option<&'a str>,
    pointer_events: Option<bool>,
    is_open: bool,
}

pub fn component<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    let is_open = cx.props.is_open;
    let opacity = if is_open {
        "opacity-100"
    } else {
        "opacity-0 select-none pointer-events-none"
    };

    let pointer_events = if cx.props.pointer_events.unwrap_or(false) {
        ""
    } else {
        "pointer-events-none"
    };

    let class = cx.props.class.unwrap_or("bg-cosmos ml-4");

    cx.render(rsx! {
        div {
            class: "hidden lg:block relative {opacity} transition delay-150 {pointer_events}",
            div {
                class: "p-4 absolute top-0 rounded {class}",
                width: "300px",
                transform: "translateY(-50%)",
                &cx.props.children
            }
        }
    })
}
