use dioxus::prelude::*;

use crate::{
    components::{
        card::{Card, RecommendList},
        navbar::NavBar,
    },
    data::model::SimpleArticle,
};

pub fn Discover(cx: Scope) -> Element {
    cx.render(rsx! {
        NavBar {}
        div {
            class: "container mx-auto",
            Card {
                h2 {
                    class: "text-2xl font-extrabold tracking-tight text-gray-900 dark:text-gray-100",
                    "Recommend Articles"
                }
                RecommendList {
                    data: vec![
                        SimpleArticle { 
                            id: 0, 
                            title: "66. 知网到底垄断了什么".to_string(), 
                            image: "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-03.jpg".to_string(), 
                            author_id: 0, 
                            create_date: 1652418229,
                        },
                        SimpleArticle { 
                            id: 0, 
                            title: "66. 知网到底垄断了什么".to_string(), 
                            image: "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-03.jpg".to_string(), 
                            author_id: 0, 
                            create_date: 1652418229,
                        },
                        SimpleArticle { 
                            id: 0, 
                            title: "66. 知网到底垄断了什么".to_string(), 
                            image: "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-03.jpg".to_string(), 
                            author_id: 0, 
                            create_date: 1652418229,
                        },
                        SimpleArticle { 
                            id: 0, 
                            title: "66. 知网到底垄断了什么".to_string(), 
                            image: "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-03.jpg".to_string(), 
                            author_id: 0, 
                            create_date: 1652418229,
                        },
                    ]
                }
            }
            br {}
        }
    })
}
