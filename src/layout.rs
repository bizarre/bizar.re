use dioxus::prelude::*;

#[derive(Props)]
pub struct Props<'a> {
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn DefaultLayout<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "w-full mt-4 flex justify-center",
            &cx.props.children
        }
    })
}
