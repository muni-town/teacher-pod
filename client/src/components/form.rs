use dioxus::prelude::*;
use dioxus_heroicons::{Icon, solid::Shape};

pub fn SearchBox(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "mt-1 relative rounded-md shadow-sm",
            div {
                class: "absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none",
                span {
                    class: "text-gray-500 sm:text-sm dark:text-white",
                    Icon {
                        icon: Shape::Search,
                        size: 15,
                    },
                }
            }
            input {
                class: "focus:ring-indigo-500 focus:border-indigo-500 block w-full pl-7 pr-12 sm:text-sm border-gray-300 rounded-md dark:bg-gray-600 dark:text-white",
                r#type: "text",
                name: "search",
            }
        }
    })
}