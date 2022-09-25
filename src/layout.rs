use dioxus::prelude::*;

#[derive(Props)]
pub struct Props<'a> {
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn DefaultLayout<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "w-full p-6 lg:p-12 pb-0 lg:pb-0 flex justify-center",
            div {
                class: "w-full lg:w-4/6 2xl:w-1/2 flex flex-col",
                &cx.props.children
            }
        }
    })
}
