use dioxus::prelude::*;

use crate::data::model::{SimpleContent, Topic};

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
    data: Vec<SimpleContent>
}

pub fn RecommendList(cx: Scope<RecommendListProps>) -> Element {
    cx.render(rsx! {
        div {
            class: "mt-6 grid grid-cols-1 gap-y-10 sm:grid-cols-2 gap-x-6 lg:grid-cols-3 xl:grid-cols-4 xl:gap-x-8",
            cx.props.data.iter().map(|item| {
                rsx! {
                    Link {
                        class: "group",
                        to: "/c/{item.id}",
                        div {
                            class: "w-full aspect-w-1 aspect-h-1 bg-gray-200 rounded-lg overflow-hidden xl:aspect-w-7 xl:aspect-h-8",
                            img {
                                class: "w-full h-full object-center object-cover group-hover:opacity-75",
                                src: "{item.cover_image}",
                            }
                        }
                        p {
                            class: "mt-1 text-lg font-medium text-gray-900 dark:text-white",
                            "{item.title}"
                        }
                        p {
                            class: "text-sm text-gray-500",
                            "{item.up_date} | YuKun Liu"
                        }
                    }
                }
            })
        }
    })
}

#[inline_props]
pub fn PopularTopics(cx: Scope, data: Vec<Topic>) -> Element {
    cx.render(rsx! {
        div {
            class: "mt-6 grid grid-cols-6 gap-x-4 gap-y-1 max-w-2xl",
            data.iter().map(|item| {
                rsx! {
                    div {
                        class: "col-span-2",
                        Link {
                            to: "/topic/{item.id}",
                            img {
                                class: "rounded-xl brightness-75",
                                src: "{item.image}"
                            }
                        }
                        p {
                            class: "text-xs -translate-y-6 text-white font-semibold sm:-translate-y-8 sm:text-base translate-x-3",
                            "{item.name}"
                        }
                    }
                }
            })
        }
    })
}