use dioxus::prelude::*;

use crate::components::card::Card;

pub fn SearchResult(cx: Scope) -> Element {

    let route = use_route(&cx);
    let query = route.query_param("query");
    if query.is_none() {
        return cx.render(rsx! { crate::pages::error::_404() });
    }
    let query = query.unwrap().to_string();

    cx.render(rsx! {
        div {
            class: "container mx-auto",
            Card {
                h1 {
                    class: "text-3xl font-semibold dark:text-white",
                    "Result for ' "
                    span {
                        class: "font-bold",
                        "{query}"
                    }
                    " '"
                }
                br {}
            }
        }
    })
}