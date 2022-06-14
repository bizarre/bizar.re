use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct Props<'a> {
    name: &'a str,
}

pub fn component<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "w-full mt-4 flex justify-center text-white",
            h1 {
                
            }
        }
    })
}
