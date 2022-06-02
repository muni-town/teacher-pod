use dioxus::prelude::*;

use crate::{
    components::{
        card::{Card, RecommendList, PopularTopics}, list::SimpleUserList,
    },
    data::model::{SimpleContent, Topic},
};

pub fn Discover(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "container mx-auto",
            Card {
                span {
                    class: "text-2xl font-extrabold tracking-tight text-gray-900 dark:text-gray-100",
                    "Recommend Contents"
                }
                RecommendList {
                    data: vec![
                        SimpleContent { 
                            id: 1, 
                            r#type: 0,
                            title: "About Us - TeacherPod".to_string(), 
                            cover_image: "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-03.jpg".to_string(), 
                            up_date: "2022-05-20".into(),
                        },
                        SimpleContent { 
                            id: 2, 
                            r#type: 0,
                            title: "About Us - TeacherPod".to_string(), 
                            cover_image: "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-03.jpg".to_string(), 
                            up_date: "2022-05-20".into(),
                        },
                    ]
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
}
