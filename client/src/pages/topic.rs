use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    components::card::{Card, RecommendList},
    data::model::{SimpleContent, Topic},
};

#[derive(Serialize, Deserialize)]
struct RequestData {
    topic: Topic,
    recommend: Vec<SimpleContent>,
}

pub fn Topic(cx: Scope) -> Element {
    let router = use_route(&cx);
    let topic_id = router.segment("id").unwrap();

    // let info: &UseFuture<Option<RequestData>> = use_future(&cx, (), |_| async move { todo!() });

    cx.render(rsx! {
        div {
            class: "container mx-auto",
            Card {
                div {
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
                    div {
                        RecommendList {
                            data: vec![
                                SimpleContent {
                                    id: 1,
                                    r#type: 1,
                                    title: "Test".into(),
                                    cover_image: "https://picsum.photos/id/10/400/400".into(),
                                    up_date: "2004-04-09".into(),
                                }
                            ]
                        }
                    }
                }
            }
        }
    })
}
