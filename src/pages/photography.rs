use dioxus::prelude::*;

use crate::{
    components::{AboutSection, Header},
    config::{instance as config, Config},
};

struct LensInfo {
    name: &'static str,
    desc: &'static str,
}

#[inline_props]
fn ImageBlock<'a>(cx: Scope, image: &'a str, class: &'a str, desc: Option<&'a str>) -> Element {
    return cx.render(rsx! {
        div {
            class: "w-full h-full group relative {class}",
            desc.is_some().then(|| {
                let desc = desc.as_ref().unwrap();
                 rsx!{
                div {
                    class: "opacity-0 transition duration-500 group-hover:opacity-100 flex group-hover:opacity-100 absolute w-full h-full text-center text-white text-opacity-50 justify-center items-center bg-black bg-opacity-50 backdrop-blur-sm cursor-pointer font-mono",
                    span {
                        class: "text-xs",
                        "{desc}"
                    }
                }
            }})
            div {
                class: "w-full h-full bg-cover bg-center aspect-square {class}",
                background_image: "url({image})",
            }
        }
    });
}

pub fn page(cx: Scope) -> Element {
    let Config { photography, .. } = &config;

    let lenses = vec![
        LensInfo {
            name: "Sigma 35mm f/1.2 DG DN ART",
            desc: "The daily. This is usually what you'll find screwed onto my camera. Versatile focal length, really good glass that, when combined with a sharp sensor, allows you to liberally crop in post and still end up with a super crisp image.",
        },
        LensInfo {
            name: "Sigma 135mm f/1.8 HSM ART",
            desc: "This lens is a ton of fun. Super sharp and I love all the cool shots you can get with the longer focal length.",
        },
        LensInfo {
            name: "Sony 20mm f/1.8 FE",
            desc: "My wide-angle prime. I don't use this one as much as I'd like, but it's really good for those wider landscape shots.",
        },
    ];

    cx.render(rsx!(
        div {
            class: "w-full flex flex-col lg:flex-row justify-between",
            div {
                div {
                    class: "w-full lg:w-500",
                    Header { name: config.name, pseudonym: config.pseudonym, headline: config.headline, social: &config.social }
                }
                div {
                    class: "w-full lg:w-500",
                    AboutSection { title: "about me", subtitle: "", text: photography.bio, span_class: "text-moss-dim" }
                    div {
                        class: "mt-3",
                        h1 {
                            class: "text-xl text-slate",
                            ""
                        }
                    }
                    div {
                        class: "p-3 px-0 w-full text-tint rounded text-lg",
                        div {
                            class: "flex items-center",
                            object {
                                class: "mr-2",
                                data:"/static/svg/camera.svg",
                                max_height: "24px",
                                max_width: "24px"
                            }
                            span {
                                "Sony "
                                span {
                                    class: "text-dim",
                                    "A7 III"
                                }
                            }
                        }
                        div {
                            p {
                                class: "text-xs text-slate",
                                "First camera I ever took a photograph with was my dad's Sony Alpha 350, have been a Sony fanboy ever since. Recently made the jump from the A6100 to this bad boy and it's a welcome upgrade. Full frame, super fast auto focus, great sensor performance, no complaints here."
                            }
                        }
                    }
                    lenses.iter().map(|lens| rsx!{
                        div {
                            class: "w-full text-dim mt-3 transition transform hover:opacity-75 hover:scale-101 cursor-pointer",
                            header {
                                class: "flex items-center",
                                object {
                                    class: "mr-1",
                                    data:"/static/svg/lens.svg",
                                    max_height: "18px",
                                    max_width: "18px"
                                }
                                span {
                                    "{lens.name}"
                                }
                            }
                            main {
                                p {
                                    class: "text-slate text-xs",
                                    "{lens.desc}"
                                }
                            }
                        }
                    })
                }
            }
            div {
                class: "w-full mt-6 pb-6 lg:pb-0 lg:mt-0 lg:ml-8",
                div {
                    class: "w-full h-full grid grid-cols-3 gap-2 grid-rows-5",
                    ImageBlock {
                        image: "/static/img/shots/entry.jpg",
                        class: "col-span-2 row-span-2",
                        desc: "boxd",
                    }
                    ImageBlock {
                        image: "/static/img/shots/lost.jpg",
                        class: "row-span-2",
                        desc: "lost"
                    }
                    ImageBlock {
                        image: "/static/img/shots/bus.jpg",
                        class: "col-span-3 aspect-auto bg-[center_top_-1.5rem]",
                        desc: "omw",
                    }
                    ImageBlock {
                        image: "/static/img/shots/tower.jpg",
                        class: "row-span-2 aspect-auto",
                        desc: "~"
                    }
                    ImageBlock {
                        image: "/static/img/shots/dim.jpg",
                        class: "",
                        desc: "~",
                    }
                    ImageBlock {
                        image: "/static/img/shots/stairs.jpg",
                        class: "row-span-2 aspect-auto",
                        desc: "~"
                    }
                    ImageBlock {
                        image: "/static/img/shots/kid.jpg",
                        class: "",
                        desc: "~"
                    }
                }
            }
        }
    ))
}
