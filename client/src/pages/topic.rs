use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    components::card::{Card, RecommendList},
    data::{
        model::{SimpleContent, Topic},
        request,
    },
};

#[derive(Serialize, Deserialize)]
struct RequestData {
    topic: Topic,
    recommend: Vec<SimpleContent>,
}

pub fn Topic(cx: Scope) -> Element {
    let router = use_route(&cx);
    let topic_id = router.segment("id").unwrap().to_string();

    let info: &UseFuture<Option<RequestData>> = use_future(&cx, (), |_| async move {
        let res = request::get(&format!("/topics/{}", topic_id))
            .send()
            .await
            .ok()?;
        let topic = res.json::<Topic>().await.ok()?;
        let res = request::get(&format!("/topics/recommend?id={}", topic_id))
            .send()
            .await
            .ok()?;
        let recommend = res.json::<Vec<SimpleContent>>().await.ok()?;
        Some(RequestData { topic, recommend })
    });

    match info.value() {
        Some(Some(info)) => {
            let topic = info.topic.clone();
            cx.render(rsx! {
                div {
                    class: "container mx-auto",
                    Card {
                        div {
                            div {
                                class: "w-full h-80",
                                img {
                                    class: "rounded-xl brightness-50 w-full h-full",
                                    src: "{topic.image}"
                                }
                                p {
                                    class: "text-5xl -translate-y-40 text-white font-bold flex justify-center",
                                    "{topic.name}"
                                }
                            }
                            div {
                                if info.recommend.is_empty() {
                                    rsx! {
                                        br { }
                                        div {
                                            class: "flex justify-center",
                                            span {
                                                class: "text-2xl text-gray-400 font-bold",
                                                "No results about \"@{topic.name}\""
                                            }
                                        }
                                    }
                                } else {
                                    rsx! {
                                        RecommendList {
                                            data: info.recommend.clone(),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            })
        },
        Some(None) => {
            cx.render(rsx! {
                crate::pages::error::_404 {}
            })
        }
        None => None,
    }

}
