use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_heroicons::{solid::Shape, Icon};

use crate::mode::{is_dark, mode};

pub fn NavBar(cx: Scope) -> Element {

    let route = use_route(&cx);
    let current_url = route.url().path();

    let default_class = "text-black dark:text-white px-3 py-2 rounded-md text-sm font-medium";
    let current_class = "bg-gray-200 dark:bg-gray-900 text-black dark:text-white px-3 py-2 rounded-md text-sm font-medium";

    let mode_icon = if *use_read(&cx, crate::DARK_MODE) {
        cx.render(rsx! { Icon { icon: Shape::Sun } })
    } else {
        cx.render(rsx! { Icon { icon: Shape::Moon } })
    };

    let set_mode = use_set(&cx, crate::DARK_MODE);

    cx.render(rsx! {
        nav {
            class: "bg-white dark:bg-gray-800",
            div {
                class: "max-w-7xl mx-auto px-2 sm:px-6 lg:px-8",
                div {
                    class: "relative flex items-center justify-between h-16",
                    div {
                        class: "absolute inset-y-0 left-0 flex items-center sm:hidden",
                        button {
                            class: "inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-white hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white",
                            "aria-controls": "mobile-menu",
                            "aria-expanded": "false",
                            r#type: "button",
                        }
                    }
                    div {
                        class: "flex-1 flex items-center justify-center sm:items-stretch sm:justify-start",
                        div {
                            class: "flex-shrink-0 flex items-center",
                            img {
                                class: "block h-8 w-auto",
                                alt: "Workflow",
                                src: "./assets/image/logo.png",
                            }
                        }
                        div {
                            class: "hidden sm:block sm:ml-6",
                            div {
                                class: "flex space-x-4",
                                Link {
                                    class: if current_url == "/" { current_class } else { default_class },
                                    to: "/",
                                    "Discover"
                                }
                                Link {
                                    class: if current_url == "/topics" { current_class } else { default_class },
                                    to: "/topics",
                                    "Topics"
                                }
                            }
                        }
                    }
                    div {
                        class: "absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0",
                        div {
                            class: "pr-6 hidden sm:block",
                            crate::components::form::SearchBox {}
                        }
                        button {
                            class: "bg-white dark:bg-gray-800 p-1 rounded-full text-gray-400 hover:bg-black hover:text-white dark:hover:bg-white dark:hover:text-black",
                            r#type: "button",
                            onclick: |_| {
                                // change current theme mode.
                                set_mode(!is_dark());
                                mode(!is_dark());
                            },
                            mode_icon
                        }
                        div {
                            class: "ml-3 relative",
                            div {
                                button {
                                    class: "bg-white dark:bg-gray-800 flex text-sm rounded-full focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-800 focus:ring-white",
                                    id: "user-menu-button",
                                    "aria-expanded": "false",
                                    "aria-haspopup": "true",
                                    r#type: "button",
                                    img {
                                        class: "h-8 w-8 rounded-full",
                                        alt: "",
                                        src: "https://avatars.githubusercontent.com/u/41265098?v=4",
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "sm:hidden",
                id: "mobile-menu",
                div {
                    class: "px-2 pt-2 pb-3 space-y-1",
                    Link {
                        class: "bg-gray-900 text-white block px-3 py-2 rounded-md text-base font-medium",
                        to: "/",
                        "Discover"
                    }
                    Link {
                        class: "text-gray-300 hover:bg-gray-700 hover:text-white block px-3 py-2 rounded-md text-base font-medium",
                        to: "/topics",
                        "Topics"
                    }
                }
            }
        }
        br {}
    })
}