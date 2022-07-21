use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_regular_icons, Icon};
use tp_models::{podcast::Podcast, ApiData};

use crate::{
    components::card::{Card, EpisodeList},
    data::request,
};

#[derive(Debug)]
struct ContentInfo {
    content: Podcast,
}

pub fn Content(cx: Scope) -> Element {
    let route = use_route(&cx);

    let id = route.segment("id").unwrap().to_string();

    let info: &UseFuture<Option<ContentInfo>> = use_future(&cx, (), |_| async move {
        let res = request::get(&format!("/podcasts/{}", id))
            .await
            .send()
            .await
            .ok()?;
        let content = res.json::<ApiData<Podcast>>().await.ok()?;
        let content = content.data;

        Some(ContentInfo { content })
    });

    log::info!("{:?}", info.value());

    // let user_info: &UseFuture<Option<Account>> =
    //     use_future(&cx, (), |_| async move { current_user().await });

    // let player_box = use_atom_ref(&cx, PLAYER_STATUS);

    match info.value() {
        Some(Some(info)) => {
            let content = &info.content;
            let description = content.description.clone();
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
                                src: "{content.thumbnail}"
                            }
                        }
                        div {
                            class: "col-span-2",
                            h1 {
                                class: "text-3xl font-semibold dark:text-white",
                                "{content.title}"
                            }
                            // p {
                            //     class: "text-lg text-gray-400",
                            //     "{content.r#type}"
                            // }
                            p {
                                class: "font-semibold text-gray-500 dark:text-gray-300 mt-4",
                                "{description}"
                            }
                            p {
                                class: "mt-6 justify-center space-x-2",
                                button {
                                    class: "inline-block px-6 py-2 border-2 border-blue-600 text-blue-600 font-medium text-xs leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 focus:outline-none focus:ring-0 transition duration-150 ease-in-out",
                                    onclick: |_| {
                                        // player_box.write().current = Some(content.clone());
                                    },
                                    Icon {
                                        icon: fa_regular_icons::FaCirclePlay,
                                    }
                                }
                                button {
                                    class: "inline-block px-6 py-2 border-2 border-blue-600 text-blue-600 font-medium text-xs leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 focus:outline-none focus:ring-0 transition duration-150 ease-in-out",
                                        Icon {
                                        icon: fa_regular_icons::FaStar,
                                    }
                                }
                            }
                        }
                        div {
                            ul {
                                class: "list-reset flex flex-col h-full",
                                li {
                                    class: "rounded-t relative -mb-px block border p-4 border-grey dark:text-white",
                                    strong { "Language : " }
                                    "{content.language}"
                                }
                                li {
                                    class: "rounded-t relative -mb-px block border p-4 border-grey dark:text-white",
                                    strong { "Total Episodes Number : " }
                                    "{content.total_episodes}"
                                }
                                li {
                                    class: "relative -mb-px block border p-4 border-grey dark:text-white",
                                    strong { "Publish Country : " }
                                    "{content.country}"
                                }
                                li {
                                    class: "relative -mb-px block border p-4 border-grey dark:text-white",
                                    strong { "Favorites Number : " }
                                    "32k"
                                }
                            }
                        }
                    }
                }
                br { }
                EpisodeList {
                    data: content.clone()
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
