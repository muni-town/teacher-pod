use dioxus::prelude::*;
use dioxus_heroicons::{Icon, solid::Shape};

use crate::components::card::Card;

pub fn User(cx: Scope) -> Element {
    let route = use_route(&cx);
    let userid = route.segment("userid").unwrap();
    cx.render(rsx! {
        div {
            class: "container mx-auto",
            Card {
                div {
                    class: "h-44",
                    div {
                        class: "flex flex-col justify-center items-center gap-2",
                        img {
                            class: "h-28 w-28 object-cover rounded-full",
                            src: "https://avatars.githubusercontent.com/u/41265098?v=4",
                        }
                        h1 {
                            class: "text-2xl font-semibold",
                            "YuKun Liu"
                        }
                    }
                }
                div {
                    div {
                        class: "grid sm:grid-cols-4 gap-4",
                        div {
                            class: "col-span-3 bg-black w-full",
                            "123"
                        }
                        div {
                            Card {
                                h1 {
                                    class: "text-lg font-semibold",
                                    "#1 - YuKun Liu"
                                }
                                div {
                                    class: "flex justify-center",
                                    ul {
                                        class: "bg-white rounded-lg w-full",
                                        li {
                                            class: "px-2 py-2 border-b border-gray-200 w-full rounded-t-lg",
                                            span {
                                                class: "font-semibold",
                                                "Gender: "
                                            }
                                            "unknown"
                                        }
                                        li {
                                            class: "px-2 py-2 border-b border-gray-200 w-full",
                                            span {
                                                class: "font-semibold",
                                                "Email: "
                                            }
                                            "mrxzx@qq.com"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}