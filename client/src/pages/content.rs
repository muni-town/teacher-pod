use dioxus::prelude::*;

use crate::{
    components::card::Card,
    data::{model, request},
};

pub fn Content(cx: Scope) -> Element {
    let route = use_route(&cx);

    let id = route.segment("id").unwrap().to_string();

    let info: &UseFuture<Option<model::Content>> = use_future(&cx, (), |_| async move {
        let res = request::get(&format!("/contents/{}", id))
            .send()
            .await
            .ok()?;
        res.json::<model::Content>().await.ok()
    });

    match info.value() {
        Some(Some(info)) => cx.render(rsx! {
            div {
                class: "container mx-auto",
               Card {
                   div {
                       class: "grid grid-cols-4 gap-4",
                       div {
                           img {
                               class: "w-full h-auto rounded-md",
                               src: "https://tailwindui.com/img/ecommerce-images/category-page-04-image-card-02.jpg"
                           }
                       }
                       div {
                           h1 {
                               class: "text-2xl font-semibold dark:text-white",
                               "About Us - TeacherPod"
                           }
                       }
                   }
               }
            }
        }),
        Some(None) => cx.render(rsx! {
            crate::pages::error::_404()
        }),
        None => None,
    }
}
