use std::sync::Arc;

use dioxus::{
    prelude::{dioxus_elements::input_data::keyboard_types::Code, *},
    router::RouterCore,
};
use dioxus_free_icons::{icons::fa_solid_icons, Icon};

pub fn SearchBox(cx: Scope) -> Element {
    let roc = cx.use_hook(|_| cx.consume_context::<Arc<RouterCore>>());
    let route = use_route(&cx);
    let text = use_state(&cx, move || {
        if route.url().path() == "/search" {
            if let Some(e) = route.query_param("query") {
                return e.to_string();
            }
        }
        String::new()
    });

    cx.render(rsx! {
        div {
            class: "mt-1 relative rounded-md shadow-sm",
            div {
                class: "absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none",
                span {
                    class: "text-gray-500 sm:text-sm dark:text-white",
                    Icon {
                        icon: fa_solid_icons::FaMagnifyingGlass,
                        size: 15,
                    },
                }
            }
            input {
                class: "focus:ring-indigo-500 focus:border-indigo-500 block w-full pl-7 pr-12 sm:text-sm border-gray-300 rounded-md dark:bg-gray-600 dark:text-white",
                r#type: "text",
                value: "{text}",
                oninput: move |evt| text.set(evt.value.clone()),
                onkeydown: move |evt| {
                    if evt.code() == Code::Enter && !text.is_empty() {
                        if let Some(roc) = roc {
                            roc.push_route(&format!("/search?query={}", text), None, None);
                        }
                    }
                },
            }
        }
    })
}
