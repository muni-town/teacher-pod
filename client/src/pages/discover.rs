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
}

pub fn Discover(cx: Scope) -> Element {

    let request_data: &UseFuture<RequestData> = use_future(&cx, (), |_| {
        async move {
            let res = request::get("/contents/").send().await;
            let res = if let Ok(resp) = res {
                resp
            } else {
                return RequestData { recommend: vec![] };
            };
            let data = res.json::<Vec<SimpleContent>>().await.unwrap_or_default();
            RequestData {
                recommend: data,
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
                                    data: vec![
                                        Topic { 
                                            id: 1001,
                                            name: "Technology".into(), 
                                            image: "https://picsum.photos/seed/2/2000/1000".into() 
                                        },
                                        Topic {
                                            id: 1002,
                                            name: "Life".into(),
                                            image: "https://picsum.photos/seed/3/2000/1000".into()
                                        },
                                        Topic {
                                            id: 1003,
                                            name: "History".into(),
                                            image: "https://picsum.photos/seed/5/2000/1000".into()
                                        }
                                    ]
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
