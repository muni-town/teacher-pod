use dioxus::prelude::*;
use serde::Deserialize;
use crate::{
    components::{
        card::{Card, RecommendList, PopularTopics}, list::SimpleUserList,
    },
    data::{model::{SimpleContent, Topic}, request},
};

#[derive(Deserialize)]
struct RequestData {
    recommend: Vec<SimpleContent>,
    popular_topics: Vec<Topic>,
}

pub fn Discover(cx: Scope) -> Element {

    let request_data: &UseFuture<RequestData> = use_future(&cx, (), |_| {
        async move {
            let res = request::get("/contents/").send().await;
            let res = if let Ok(resp) = res {
                resp
            } else {
                return RequestData { recommend: vec![], popular_topics: vec![] };
            };
            let recommend = res.json::<Vec<SimpleContent>>().await.unwrap_or_default();
            
            let res = request::get("/topics/").send().await;
            let res = if let Ok(resp) = res {
                resp
            } else {
                return RequestData { recommend: vec![], popular_topics: vec![] };
            };
            let popular_topics = res.json::<Vec<Topic>>().await.unwrap_or_default();
            
            RequestData {
                recommend,
                popular_topics
            }
        }
    });

    match request_data.value() {
        Some(v) => {
            cx.render(rsx! {
                div {
                    class: "container mx-auto",
                    Card {
                        span {
                            class: "text-2xl font-extrabold tracking-tight text-gray-900 dark:text-gray-100",
                            "Recommend Contents"
                        }
                        RecommendList {
                            data: v.recommend.clone(),
                        }
                    }
                    br {}
                    div {
                        class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                        div {
                            class: "md:col-span-2",
                            Card {
                                h2 {
                                    class: "text-2xl font-extrabold tracking-tight text-gray-900 dark:text-gray-100",
                                    "Popular Topics"
                                }
                                PopularTopics {
                                    data: v.popular_topics.clone(),
                                }
                            }
                        }
                        div {
                            class: "md:col-span-1",
                            Card {
                                h2 {
                                    class: "text-2xl font-extrabold tracking-tight text-gray-900 dark:text-gray-100",
                                    "Popular Users"
                                }
                                div {
                                    class: "mt-5",
                                    SimpleUserList {
                                        data: vec![]
                                    }
                                }
                            }
                        }
                    }
                }
            })
        },
        None => None
    }

}
