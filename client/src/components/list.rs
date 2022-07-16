use dioxus::prelude::*;
use tp_models::{account::Account, data::SearchInfo};

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
                div {
                    class: "rounded-xl border p-5 shadow-md w-9/12 bg-white",
                    div {
                        class: "flex w-full items-center justify-between border-b pb-3",
                        div {
                            class: "flex items-center space-x-3",
                            div {
                                class: "h-8 w-8 rounded-full bg-slate-400 bg-[url('https://i.pravatar.cc/32')]",
                                
                            }
                            div {
                                class: "text-lg font-bold text-slate-700",
                                "Joe Smith"
                            }
                        }
                        div {
                            class: "flex items-center space-x-8",
                            button {
                                class: "rounded-2xl border bg-neutral-100 px-3 py-1 text-xs font-semibold",
                                "Category"
                            }
                            div {
                                class: "text-xs text-neutral-500",
                                "2 hours ago"
                            }
                        }
                    }
                    div {
                        class: "mt-4 mb-6",
                        div {
                            class: "mb-3 text-xl font-bold",
                            "Nulla sed leo tempus, feugiat velit vel, rhoncus neque?"
                        }
                        div {
                            class: "text-sm text-neutral-600",
                            "Aliquam a tristique sapien, nec bibendum urna. Maecenas convallis dignissim turpis, non suscipit mauris interdum at. Morbi sed gravida nisl, a pharetra nulla. Etiam tincidunt turpis leo, ut mollis ipsum consectetur quis. Etiam faucibus est risus, ac condimentum mauris consequat nec. Curabitur eget feugiat massa"
                        }
                    }
                    div {
                        div {
                            class: "flex items-center justify-between text-slate-500",
                            div {
                                class: "flex space-x-4 md:space-x-8",
                            }
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