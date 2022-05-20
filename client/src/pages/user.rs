use dioxus::prelude::*;

use crate::{
    components::card::Card,
    data::{model::SimpleUser, request},
};

pub fn User(cx: Scope) -> Element {
    let route = use_route(&cx);
    let userid = route.segment("userid").unwrap().to_string();

    let user_info: &UseFuture<Option<SimpleUser>> = use_future(&cx, (), |_| async move {
        let resp = request::get(&format!("/users/{}", userid))
            .send()
            .await
            .ok()?;
        resp.json::<SimpleUser>().await.ok()
    });

    match user_info.value() {
        Some(Some(user)) => {
            cx.render(rsx! {
                div {
                    class: "container mx-auto",
                    Card {
                        div {
                            class: "h-44",
                            div {
                                class: "flex flex-col justify-center items-center gap-2",
                                img {
                                    class: "h-28 w-28 object-cover rounded-full",
                                    src: "https://avatars.githubusercontent.com/u/41265098?v=4",
                                }
                                h1 {
                                    class: "text-2xl font-semibold dark:text-white",
                                    "YuKun Liu"
                                }
                            }
                        }
                        div {
                            div {
                                class: "grid sm:grid-cols-4 gap-4 dark:text-white",
                                div {
                                    class: "col-span-3 bg-gray-400 w-full",
                                    "123"
                                }
                                div {
                                    Card {
                                        h1 {
                                            class: "text-lg font-semibold dark:text-white",
                                            "#{user.id} - {user.username}"
                                        }
                                        div {
                                            class: "flex justify-center",
                                            ul {
                                                class: "bg-white dark:bg-gray-800 rounded-lg w-full",
                                                li {
                                                    class: "px-2 py-2 border-b border-gray-200 w-full rounded-t-lg",
                                                    span {
                                                        class: "font-semibold",
                                                        "Gender: "
                                                    }
                                                    "{user.gender}"
                                                }
                                                li {
                                                    class: "px-2 py-2 border-b border-gray-200 w-full",
                                                    span {
                                                        class: "font-semibold",
                                                        "Email: "
                                                    }
                                                    "{user.email}"
                                                }
                                                li {
                                                    class: "px-2 py-2 border-b border-gray-200 w-full",
                                                    span {
                                                        class: "font-semibold",
                                                        "Reg-Date: "
                                                    }
                                                    "{user.reg_date}"
                                                }
                                                li {
                                                    class: "px-2 py-2 border-b border-gray-200 w-full",
                                                    span {
                                                        class: "font-semibold",
                                                        "Bio: "
                                                    }
                                                    "Not Found..."
                                                }
                                            }
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
                crate::pages::error::_404()
            })
        },
        None => {
            None
        },
    }
}
