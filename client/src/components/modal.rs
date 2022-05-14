use dioxus::prelude::*;
use dioxus_heroicons::{Icon, solid::Shape};

pub fn PlayBox(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "hidden sm:block fixed bottom-8 left-0 w-1/4 h-20 rounded
            bg-white dark:bg-black shadow-2xl
            px-2 py-2 z-40
            ",
            div {
                class: "flex h-full gap-2",
                div {
                    class: "flex-none",
                    img {
                        class: "h-full rounded",
                        src: "https://picsum.photos/seed/2/200/200",
                    }
                }
                div {
                    class: "grow relative",
                    div {
                        span {
                            class: "text-black dark:text-white",
                            "66. About Us - TeacherPod"
                        }
                        span {
                            class: "absolute right-0",
                            div {
                                class: "flex items-center justify-center",
                                div {
                                    class: "inline-flex",
                                    role: "group",
                                    button {
                                        class: "rounded-full inline-block px-1 py-1 text-black dark:text-white font-medium text-xs leading-tight hover:bg-gray-800 hover:text-white transition duration-150 ease-in-out",
                                        r#type: "button",
                                        Icon {
                                            icon: Shape::Pause
                                        }   
                                    }
                                    button {
                                        class: "rounded-full inline-block px-1 py-1 text-black dark:text-white font-medium text-xs leading-tight hover:bg-gray-800 hover:text-white transition duration-150 ease-in-out",
                                        r#type: "button",
                                        Icon {
                                            icon: Shape::X
                                        }   
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "absolute bottom-0 w-full",
                        input {
                            class: "w-full h-1",
                            r#type: "range",
                            min: "0",
                            max: "60",
                            step: "1",
                            value: "402"
                        }
                    }
                }
            }
        }
    })
}