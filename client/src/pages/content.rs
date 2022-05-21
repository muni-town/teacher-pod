use dioxus::prelude::*;

use crate::components::card::Card;

pub fn Content(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "container mx-auto",
           Card {
               "123"
           }
        }
    })
}