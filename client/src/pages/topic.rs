use dioxus::prelude::*;

use crate::components::card::Card;

pub fn Topic(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "container mx-auto",
            Card {
                div {
                    class: "flex justify-center",
                    div {
                        class: "w-80 h-44",
                        img {
                            class: "rounded-xl brightness-75",
                            src: "https://picsum.photos/seed/2/2000/1000"
                        }
                    }

                }
                div {
                    class: "flex justify-center",
                    p {
                        class: "font-semibold text-2xl",
                        "Technology"
                    }
                }
            }
        }
    })
}