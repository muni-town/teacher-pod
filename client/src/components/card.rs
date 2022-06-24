use dioxus::prelude::*;
use dioxus_heroicons::{solid::Shape, Icon};
use tp_models::podcast::{BestPodcasts, Episode};

#[inline_props]
pub fn Card<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    cx.render(rsx! {
        div {
            class: "bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg",
            div {
                class: "px-4 py-5 sm:px-6",
                children
            }
        }
    })
}

#[derive(Props, PartialEq)]
pub struct RecommendListProps {
    data: BestPodcasts,
}

pub fn RecommendList(cx: Scope<RecommendListProps>) -> Element {
    cx.render(rsx! {
        div {
            class: "mt-6 grid grid-cols-1 gap-y-10 sm:grid-cols-2 gap-x-6 lg:grid-cols-3 xl:grid-cols-4 xl:gap-x-8",
            cx.props.data.podcasts.iter().map(|item| {
                rsx! {
                    Link {
                        class: "group",
                        to: "/conent/{item.id}",
                        div {
                            class: "w-full aspect-w-1 aspect-h-1 bg-gray-200 rounded-lg overflow-hidden xl:aspect-w-7 xl:aspect-h-8",
                            img {
                                class: "w-full h-full object-center object-cover group-hover:opacity-75",
                                src: "{item.thumbnail}",
                            }
                        }
                        p {
                            class: "mt-1 text-lg font-medium text-gray-900 dark:text-white",
                            "{item.title}"
                        }
                        p {
                            class: "text-sm text-gray-500",
                            "{item.total_episodes}"
                        }
                    }
                }
            })
        }
    })
}

#[inline_props]
pub fn EpisodeList(cx: Scope, data: Vec<Episode>) -> Element {
    cx.render(rsx! {
        Card {
            data.iter().map(|item| {
                let min: f32 = (item.audio_length_sec as f32) / 60_f32;
                rsx! {
                    a {
                        class: "
                        block
                        px-6
                        py-4
                        border-b border-gray-200
                        w-full
                        text-dark
                        hover:bg-gray-100
                        hover:text-gray-500
                        dark:text-white
                        dark:hover:bg-gray-700
                        dark:hover:text-gray-100
                        cursor-pointer
                        ",
                        href: "/12312",
                        Icon {
                            class: "float-left text-gray-600",
                            size: 24,
                            icon: Shape::Play
                        }
                        strong {
                            "{item.title}"
                        }
                        span {
                            class: "float-right text-gray-400",
                            "{min:.1} min"
                        }
                    }
                }
            })
        }
    })
}

#[inline_props]
pub fn PopularTopics(cx: Scope, data: Vec<String>) -> Element {
    cx.render(rsx! {
        div {
            class: "mt-6 grid grid-cols-6 gap-x-4 gap-y-1 max-w-2xl",
            data.iter().map(|item| {
                rsx! {
                    div {
                        class: "col-span-2",
                        Link {
                            to: "/topic/{item}",
                            img {
                                class: "rounded-xl brightness-75",
                                src: "{item}"
                            }
                        }
                        p {
                            class: "text-xs -translate-y-6 text-white font-semibold sm:-translate-y-8 sm:text-base translate-x-3",
                            "{item}"
                        }
                    }
                }
            })
        }
    })
}
