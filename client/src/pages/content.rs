use dioxus::prelude::*;

use crate::{
    components::card::Card,
    data::{model, request},
};

pub fn Content(cx: Scope) -> Element {
    let route = use_route(&cx);

    let id = route.segment("id").unwrap().to_string();

    let info: &UseFuture<Option<model::Content>> = use_future(&cx, (), |_| async move {
        let res = request::get(&format!("/contents/{}", id))
            .send()
            .await
            .ok()?;
        res.json::<model::Content>().await.ok()
    });

    match info.value() {
        Some(Some(info)) => {
            let description = info.description.clone();
            let description = if description.len() > 350 {
                format!("{}...", &description[0..349])
            } else {
                description
            };

            cx.render(rsx! {
                div {
                class: "container mx-auto",
                Card {
                    div {
                        class: "grid grid-cols-4 gap-4",
                        div {
                            class: "col-span-1",
                            img {
                                class: "w-full h-auto rounded-md",
                                src: "{info.cover_image}"
                            }
                        }
                        div {
                            class: "col-span-2",
                            h1 {
                                class: "text-3xl font-semibold dark:text-white",
                                "{info.title}"
                            }
                            p {
                                class: "text-lg text-gray-400",
                                Link {
                                    class: "hover:text-blue-500",
                                    to: "/u/{info.author.id}",
                                    "{info.author.username}"
                                }
                                " | {info.up_date}"
                            }
                            p {
                                class: "font-semibold text-gray-500 dark:text-gray-300 h-[140px]",
                                "{description}"
                            }
                            p {
                                class: "",
                                "123213123"
                            }
                        }
                    }
                }
                }
            })
        },
        Some(None) => cx.render(rsx! {
            crate::pages::error::_404()
        }),
        None => None,
    }
}
