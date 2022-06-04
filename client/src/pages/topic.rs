use dioxus::prelude::*;

use crate::components::card::Card;

pub fn Topic(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "container mx-auto",
            Card {
                div {
                    class: "grid grid-rows-3 grid-flow-col gap-4",
                    div {
                        class: "w-full h-80",
                        img {
                            class: "rounded-xl brightness-50 w-full h-full",
                            src: "https://picsum.photos/seed/2/2000/1000"
                        }
                        p {
                            class: "text-5xl -translate-y-40 text-white font-bold flex justify-center",
                            "Technology"
                        }
                    }
                }
            }
        }
    })
}