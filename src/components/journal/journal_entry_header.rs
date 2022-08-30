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
        header {
            class: "flex flex-col mb-4 w-full",
            entry.is_none().then(|| rsx!(
                div {
                    class: "w-full lg:w-1/2 skeleton-super-light self-center h-6 rounded",
                }
                div {
                    class: "w-full flex justify-between mt-2",
                    div {
                        class: "w-20 skeleton h-4 rounded"
                    }
                    div {
                        class: "w-20 skeleton h-4 rounded"
                    }
                }
            ))
            entry.is_some().then(|| {
                let entry = entry.clone().unwrap();
                rsx!(
                    h1 {
                        class: "text-white self-center text-xl font-semibold",
                        "{entry.title}"
                    }
                    div {
                        class: "flex justify-between",
                        h2 {
                            class: "text-slate self-end font-medium text-sm",
                            "{entry.subtitle}"
                        }
                        label {
                            class: "text-slate self-end font-medium text-sm",
                            "{entry.date}"
                        }
                    }
                )
            })
        }
    ))
}
