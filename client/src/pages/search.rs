use dioxus::prelude::*;
use serde::Deserialize;
use tp_models::{data::SearchInfo, ApiData};

use crate::{
    components::{card::Card, list::SearchResultList},
    data::request,
};

#[derive(Deserialize, Debug)]
struct RequestData {
    result: Option<SearchInfo>,
}

pub fn SearchResult(cx: Scope) -> Element {
    let route = use_route(&cx);
    let query = route.query_param("query");
    if query.is_none() {
        return cx.render(rsx! { crate::pages::error::_404() });
    }
    let query = query.unwrap().to_string();

    let query_str = query.clone();
    let request_data = use_future(&cx, (), move |_| async move {
        let res = request::get(&format!("/search/{}", query_str)).await.send().await;
        let res = if let Ok(resp) = res {
            resp
        } else {
            return RequestData { result: None };
        };

        let result = res.json::<ApiData<SearchInfo>>().await.unwrap_or_default();
        let result = result.data;
        RequestData {
            result: Some(result),
        }
    });


    match request_data.value() {
        Some(data) => {
            let data = data.result.as_ref();
            if data.is_none() {
                return None;
            }
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
                        hr {}
                        br {}
                        SearchResultList {
                            data: data.unwrap().clone()
                        }
                    }
                }
            })
        },
        None => None,
    }
}
