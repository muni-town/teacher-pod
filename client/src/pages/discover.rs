use crate::{
    components::card::{Card, RecommendList},
    data::request,
};
use dioxus::prelude::*;
use serde::Deserialize;
use tp_models::podcast::BestPodcasts;

#[derive(Deserialize, Debug)]
struct RequestData {
    recommend: Option<BestPodcasts>,
}

pub fn Discover(cx: Scope) -> Element {
    let request_data: &UseFuture<RequestData> = use_future(&cx, (), |_| {
        async move {
            let res = request::get("/podcasts/").send().await;
            let res = if let Ok(resp) = res {
                resp
            } else {
                return RequestData { recommend: None };
            };
            log::info!("{}", res.text().await.unwrap());
            let recommend = res.json::<BestPodcasts>().await.unwrap_or_default();
            // let res = request::get("/topics/").send().await;
            // let res = if let Ok(resp) = res {
            //     resp
            // } else {
            //     return RequestData { recommend: vec![], popular_topics: vec![] };
            // };
            // let popular_topics = res.json::<Vec<Topic>>().await.unwrap_or_default();

            RequestData { recommend: Some(recommend) }
        }
    });
    log::info!("{:?}", request_data.value());

    match request_data.value() {
        Some(v) => {
            cx.render(rsx! {
                div {
                    class: "container mx-auto",
                    Card {
                        span {
                            class: "text-2xl font-extrabold tracking-tight text-gray-900 dark:text-gray-100",
                            "Recommend Podcasts"
                        }
                        if v.recommend.is_some() {
                            rsx! {
                                RecommendList {
                                    data: v.recommend.clone().unwrap(),
                                }
                            }
                        } else {
                            rsx! { div { "Not Found" } }
                        }
                    }
                    br {}
                    // div {
                    //     class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                    //     div {
                    //         class: "md:col-span-2",
                    //         Card {
                    //             h2 {
                    //                 class: "text-2xl font-extrabold tracking-tight text-gray-900 dark:text-gray-100",
                    //                 "Popular Topics"
                    //             }
                    //             div {
                    //                 class: "flex justify-center",
                    //                 PopularTopics {
                    //                     data: v.popular_topics.clone(),
                    //                 }
                    //             }
                    //         }
                    //     }
                    //     div {
                    //         class: "md:col-span-1",
                    //         Card {
                    //             h2 {
                    //                 class: "text-2xl font-extrabold tracking-tight text-gray-900 dark:text-gray-100",
                    //                 "Popular Users"
                    //             }
                    //             div {
                    //                 class: "mt-5",
                    //                 SimpleUserList {
                    //                     data: vec![]
                    //                 }
                    //             }
                    //         }
                    //     }
                    // }
                }
            })
        }
        None => None,
    }
}
