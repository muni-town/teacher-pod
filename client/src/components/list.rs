use dioxus::prelude::*;
use tp_models::account::Account;


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