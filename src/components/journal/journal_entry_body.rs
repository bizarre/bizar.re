use crate::pages::JournalEntry;

use dioxus::{prelude::*, router::*};

#[derive(Props, PartialEq)]
pub struct Props {
    #[props(!optional)]
    entry: Option<JournalEntry>,
}

pub fn component<'a>(cx: Scope<'a, Props>) -> Element {
    let entry = &cx.props.entry;
    cx.render(rsx!(
        section {
            class: "flex items-end flex-col w-full",
            entry.is_none().then(|| rsx!(
                div {
                    class: "w-full",
                    div {
                        class: "skeleton-light w-full h-4 mt-2 rounded"
                    }
                    div {
                        style: "width: 96%",
                        class: "skeleton-light w-full h-4 mt-2 rounded"
                    }
                    div {
                        style: "width: 91%",
                        class: "skeleton-light w-full h-4 mt-2 rounded"
                    }
                    div {
                        class: "skeleton-light w-full h-4 mt-2 rounded"
                    }
                    div {
                        style: "width: 95%",
                        class: "skeleton-light w-full h-4 mt-2 rounded"
                    }
                    div {
                        style: "width: 89%",
                        class: "skeleton-light w-full h-4 mt-6 rounded"
                    }
                    div {
                        class: "skeleton-light w-full h-4 mt-2 rounded"
                    }
                    div {
                        style: "width: 93%",
                        class: "skeleton-light w-full h-4 mt-2 rounded"
                    }
                    div {
                        style: "width: 90%",
                        class: "skeleton-light w-full h-4 mt-2 rounded"
                    }
                    div {
                        class: "skeleton-light w-full h-4 mt-2 rounded"
                    }
                    div {
                        style: "width: 96%",
                        class: "skeleton-light w-full h-4 mt-6 rounded"
                    }
                    div {
                        style: "width: 90%",
                        class: "skeleton-light w-full h-4 mt-2 rounded"
                    }
                    div {
                        class: "skeleton-light w-full h-4 mt-2 rounded"
                    }
                    div {
                        style: "width: 80%",
                        class: "skeleton-light w-full h-4 mt-2 rounded"
                    }
                    div {
                        style: "width: 100%",
                        class: "skeleton-light w-full h-4 mt-2 rounded"
                    }
                    div {
                        style: "width: 70%",
                        class: "skeleton-light w-full h-4 mt-2 rounded"
                    }
                }
            ))
            entry.is_some().then(|| {
                let entry = entry.clone().unwrap();
                rsx!(
                    div {
                        class: "prose prose-sm prose-stone",
                        style: "max-width: inherit;",
                        dangerous_inner_html: "{entry.body}",
                    }
                )
            })
        }
    ))
}
