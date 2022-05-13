use dioxus::prelude::*;

use crate::{
    components::{
        card::{Card, RecommendList, PopularTopics},
    },
    data::model::{SimpleArticle, Topic},
};

pub fn Discover(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "container mx-auto",
            Card {
                span {
                    class: "text-2xl font-extrabold tracking-tight text-gray-900 dark:text-gray-100",
                    "Recommend Articles"
                }
                RecommendList {
                    data: vec![
                        SimpleArticle { 
                            id: 1000, 
                            title: "66. 知网到底垄断了什么".to_string(), 
                            image: "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-03.jpg".to_string(), 
                            author_id: 0, 
                            create_date: 1652418229,
                        },
                        SimpleArticle { 
                            id: 1001, 
                            title: "66. 知网到底垄断了什么".to_string(), 
                            image: "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-03.jpg".to_string(), 
                            author_id: 0, 
                            create_date: 1652418229,
                        },
                        SimpleArticle { 
                            id: 1002, 
                            title: "66. 知网到底垄断了什么".to_string(), 
                            image: "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-03.jpg".to_string(), 
                            author_id: 0, 
                            create_date: 1652418229,
                        },
                        SimpleArticle { 
                            id: 1003, 
                            title: "66. 知网到底垄断了什么".to_string(), 
                            image: "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-03.jpg".to_string(), 
                            author_id: 0, 
                            create_date: 1652418229,
                        },
                    ]
                }
            }
            br {}
            div {
                class: "grid grid-cols-3 gap-4",
                div {
                    class: "col-span-2",
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
                    class: "col-span-1",
                    Card {}
                }
            }
        }
    })
}
