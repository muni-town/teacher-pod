use dioxus::prelude::*;
use dioxus_heroicons::{solid::Shape, Icon};

use crate::{
    components::card::Card,
    data::{
        account::current_user,
        model::{self, SimpleUser},
        request,
    },
    PLAYER_STATUS,
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

    let user_info: &UseFuture<Option<SimpleUser>> =
        use_future(&cx, (), |_| async move { current_user().await });

    let player_box = use_atom_ref(&cx, PLAYER_STATUS);

    match info.value() {
        Some(Some(info)) => {
            let description = info.description.clone();
            let description = if description.len() > 350 {
                format!("{}...", &description[0..349])
            } else {
                description
            };

            let button_list: Element = {
                // check user login
                let v = user_info.value();
                if v.is_some() && v.unwrap().is_some() && v.unwrap().clone().unwrap().id == info.author.id {
                    cx.render(rsx! {
                        button {
                            class: "inline-block px-6 py-2 border-2 border-blue-600 text-blue-600 font-medium text-xs leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 focus:outline-none focus:ring-0 transition duration-150 ease-in-out",
                            "Edit"
                        }
                    })
                } else {
                    cx.render(rsx! {
                        button {
                            class: "inline-block px-6 py-2 border-2 border-yellow-500 text-yellow-500 font-medium text-xs leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 focus:outline-none focus:ring-0 transition duration-150 ease-in-out",
                            "Report"
                        }
                    })
                }
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
                                class: "font-semibold text-gray-500 dark:text-gray-300 mt-4",
                                "{description}"
                            }
                            p {
                                class: "mt-6 justify-center space-x-2",
                                button {
                                    class: "inline-block px-6 py-2 border-2 border-blue-600 text-blue-600 font-medium text-xs leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 focus:outline-none focus:ring-0 transition duration-150 ease-in-out",
                                    onclick: |_| {
                                        player_box.write().current = Some(info.clone());
                                    },
                                    Icon {
                                        icon: Shape::Play
                                    }
                                }
                                button {
                                    class: "inline-block px-6 py-2 border-2 border-blue-600 text-blue-600 font-medium text-xs leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 focus:outline-none focus:ring-0 transition duration-150 ease-in-out",
                                        Icon {
                                        icon: Shape::Star
                                    }
                                }
                            }
                        }
                        div {
                            ul {
                                class: "list-reset flex flex-col h-full",
                                li {
                                    class: "rounded-t relative -mb-px block border p-4 border-grey dark:text-white",
                                    strong { "Publish User : " }
                                    Link {
                                        class: "hover:text-blue-500 underline"
                                        to: "/u/{info.author.id}",
                                        "{info.author.username}"
                                    }
                                }
                                li {
                                    class: "relative -mb-px block border p-4 border-grey dark:text-white",
                                    strong { "Publish Date : " }
                                    "{info.up_date}"
                                }
                                li {
                                    class: "relative -mb-px block border p-4 border-grey dark:text-white",
                                    strong { "Favorites Number : " }
                                    "32"
                                }
                                li {
                                    class: "rounded-b relative block border p-4 border-grey dark:text-white",
                                    div {
                                        class: "flex space-x-2",
                                        button_list
                                    }
                                }
                            }
                        }
                    }
                }
                }
            })
        }
        Some(None) => cx.render(rsx! {
            crate::pages::error::_404()
        }),
        None => None,
    }
}
