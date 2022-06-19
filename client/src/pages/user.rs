use dioxus::prelude::*;
use dioxus_heroicons::{Icon, solid::Shape};
use tp_models::account::Account;

use crate::{
    components::card::Card,
    data::{request, account::current_user},
};

pub fn User(cx: Scope) -> Element {
    let route = use_route(&cx);
    let userid = route.segment("userid").unwrap().to_string();

    let current_user_page = use_state(&cx, || false);

    let user_info: &UseFuture<Option<Account>> = use_future(&cx, (), |_| {
        let current_user_page = current_user_page.clone();
        async move {
        
            if let Some(u) = current_user().await {
                current_user_page.set(true);
                return Some(u);
            }
    
            let resp = request::get(&format!("/users/{}", userid))
                .send()
                .await
                .ok()?;
            resp.json::<Account>().await.ok()
        }
    });

    match user_info.value() {
        Some(Some(user)) => {
            cx.render(rsx! {
                div {
                    class: "container mx-auto",
                    Card {
                        div {
                            class: "h-56",
                            div {
                                class: "flex flex-col justify-center items-center gap-3",
                                img {
                                    class: "h-28 w-28 object-cover rounded-full",
                                    src: "{user.avatar}",
                                }
                                h1 {
                                    class: "text-2xl font-semibold dark:text-white",
                                    "YuKun Liu"
                                }
                                if *current_user_page.get() {
                                    rsx! {
                                        div {
                                            class: "
                                            inline-block 
                                            px-6 
                                            py-2 
                                            border-2 
                                            border-blue-600 
                                            text-blue-600 
                                            font-medium 
                                            text-xs 
                                            leading-tight 
                                            uppercase 
                                            rounded 
                                            hover:bg-black 
                                            hover:bg-opacity-5 
                                            focus:outline-none 
                                            focus:ring-0 
                                            transition 
                                            duration-150 
                                            ease-in-out
                                            ",
                                            "edit info"
                                        }
                                    }
                                } else {
                                    rsx! {
                                        div {
                                            class: "
                                            inline-block 
                                            px-6 
                                            py-2 
                                            border-2 
                                            border-blue-600 
                                            text-blue-600 
                                            font-medium 
                                            text-xs 
                                            leading-tight 
                                            uppercase 
                                            rounded 
                                            hover:bg-black 
                                            hover:bg-opacity-5 
                                            focus:outline-none 
                                            focus:ring-0 
                                            transition 
                                            duration-150 
                                            ease-in-out
                                            ",
                                            "follow"
                                        }
                                    }
                                }
                            }
                        }
                        div {
                            div {
                                class: "grid sm:grid-cols-4 gap-4 dark:text-white",
                                div {
                                    class: "col-span-3",
                                    ol {
                                        class: "border-l-2 border-purple-600",
                                        li {
                                            div {
                                                class: "md:flex flex-start",
                                                div {
                                                    class: "bg-purple-600 w-6 h-6 flex items-center justify-center rounded-full -ml-3 text-white",
                                                    Icon {
                                                        icon: Shape::Star
                                                    }
                                                }
                                                div {
                                                    class: "block p-6 rounded-lg shadow-lg bg-gray-100 dark:bg-gray-900 ml-6 mb-10 w-full",
                                                    div {
                                                        class: "flex justify-between mb-4",
                                                        a {
                                                            class: "font-medium text-purple-600 hover:text-purple-700 focus:text-purple-800 duration-300 transition ease-in-out text-sm",
                                                            href: "#",
                                                            "New Web Design"
                                                        }
                                                        a {
                                                            class: "font-medium text-purple-600 hover:text-purple-700 focus:text-purple-800 duration-300 transition ease-in-out text-sm",
                                                            href: "#",
                                                            "04 / 02 / 2022"
                                                        }
                                                    }
                                                    p {
                                                        class: "text-gray-700 dark:text-white mb-6",
                                                        "this a test content"
                                                    }
                                                    button {
                                                        class: "inline-block px-4 py-1.5 bg-purple-600 text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-purple-700 hover:shadow-lg focus:bg-purple-700 focus:shadow-lg focus:outline-none focus:ring-0 active:bg-purple-800 active:shadow-lg transition duration-150 ease-in-out",
                                                        "data-mdb-ripple": "true",
                                                        r#type: "button",
                                                        "Preview"
                                                    }
                                                }
                                            }
                                        }
                                    }
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
                                                    "{user.introduction}"
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
