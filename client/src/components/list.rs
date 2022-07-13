use dioxus::prelude::*;
use tp_models::{account::Account, data::SearchInfo};

use super::card::Card;


#[inline_props]
pub fn SimpleUserList(cx: Scope, data: Vec<Account>) -> Element {
    cx.render(rsx! {
        ul {
            class: "divide-y divide-gray-200 dark:divide-gray-700",
            role: "list",
            data.iter().map(|item| {
                rsx! {
                    li {
                        class: "py-3 sm:py-4",
                        div {
                            class: "flex items-center space-x-4",
                            div {
                                class: "flex-shrink-0",
                                img {
                                    class: "w-8 h-8 rounded-full",
                                    alt: "Neil image",
                                    src: "{item.avatar}",
                                }
                            }
                            div {
                                class: "flex-1 min-w-0",
                                p {
                                    class: "text-sm font-medium text-gray-900 truncate dark:text-white",
                                    "{item.username}"
                                }
                                p {
                                    class: "text-sm text-gray-500 truncate dark:text-gray-400",
                                    "{item.email}"
                                }
                            }
                        }
                    }
                }
            })
        }
    })
}

#[inline_props]
pub fn SearchResultList(cx: Scope, data: SearchInfo) -> Element {

    let list_display = data.results.iter().map(|v| {
        let title = v.title_original.clone();
        rsx! {
            a {
                href: "https://google.com",
                class: "bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg hover:bg-gray-200 dark:hover:bg-gray-600 text-balck dark:text-white",
                div {
                    class: "px-4 py-5 sm:px-6",
                    h3 {
                        class: "text-lg font-semibold",
                        "{title}"
                    }
                    img {
                        class: "w-10 h-108",
                        src: "{v.image}"
                    }
                }
            }
        }
    });

    cx.render(rsx! {
        div {
            class: "flex flex-col space-y-4",
            list_display
        }
    })
}