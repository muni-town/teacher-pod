use dioxus::prelude::*;
use dioxus_toast::ToastInfo;

use crate::{data::account::login, TOAST_MANAGER};

pub fn Login(cx: Scope) -> Element {

    let toast = use_atom_ref(&cx, TOAST_MANAGER);

    let email = use_state(&cx, String::new);
    let password = use_state(&cx, String::new);

    cx.render(rsx! {
        div {
            class: "justify-center",
            div {
                class: "p-10 xs:p-0 mx-auto md:w-full md:max-w-md",
                h1 {
                    class: "font-bold text-center text-black dark:text-white text-2xl mb-5",
                    "Sign In | TeacherPod"
                }
                div {
                    class: "bg-white dark:bg-gray-800 shadow w-full rounded-lg divide-y divide-gray-200",
                    div {
                        class: "px-5 py-7",
                        label {
                            class: "font-semibold text-sm text-gray-600 dark:text-gray-50 pb-1 block",
                            "E-mail"
                        }
                        input {
                            class: "border rounded-lg px-3 py-2 mt-1 mb-5 text-sm w-full",
                            r#type: "text",
                            oninput: move |e| email.set(e.value.clone()),
                        }
                        label {
                            class: "font-semibold text-sm text-gray-600 dark:text-gray-50 pb-1 block",
                            "Password"
                        }
                        input {
                            class: "border rounded-lg px-3 py-2 mt-1 mb-5 text-sm w-full",
                            r#type: "text",
                            oninput: move |e| password.set(e.value.clone()),
                        }
                        button {
                            class: "transition duration-200 bg-blue-500 hover:bg-blue-600 focus:bg-blue-700 focus:shadow-sm focus:ring-4 focus:ring-blue-500 focus:ring-opacity-50 text-white w-full py-2.5 rounded-lg text-sm shadow-sm hover:shadow-md font-semibold text-center inline-block",
                            r#type: "button",
                            onclick: move |_| {
                                let email = email.get().clone();
                                let password = password.get().clone();
                                let toast = toast.clone();
                                cx.spawn(async move {

                                    if !email.is_empty() && !password.is_empty() {
                                        let res = login(&email, &password).await;
                                        if let Err(e) = res {
                                            toast.write().popup(ToastInfo { 
                                                heading: None, 
                                                context: e.to_string(), 
                                                allow_toast_close: true, 
                                                position: dioxus_toast::Position::BottomRight, 
                                                icon: Some(dioxus_toast::Icon::Error), 
                                                hide_after: Some(4),
                                            });
                                        } else {
                                            toast.write().popup(ToastInfo::success("OK", "OK"));
                                        }
                                    }
                                });
                            },
                            span {
                                class: "inline-block mr-2",
                                "Login"
                            }
                        }
                    }
                    div {
                        class: "py-5",
                        div {
                            class: "grid grid-cols-2 gap-1",
                            div { }
                            div {
                                class: "text-center sm:text-right whitespace-nowrap",
                                Link {
                                    class: "transition duration-200 mx-5 px-5 py-4 cursor-pointer font-normal text-sm rounded-lg text-gray-500 dark:text-white hover:bg-gray-100 dark:hover:text-black focus:outline-none focus:bg-gray-200 focus:ring-2 focus:ring-gray-400 focus:ring-opacity-50 ring-inset",
                                    to: "/register",
                                    span {
                                        class: "inline-block ml-1",
                                        "Sign Up"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

pub fn Register(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "justify-center",
            div {
                class: "p-10 xs:p-0 mx-auto md:w-full md:max-w-md",
                h1 {
                    class: "font-bold text-center text-black dark:text-white text-2xl mb-5",
                    "Sign Up | TeacherPod"
                }
                div {
                    class: "bg-white dark:bg-gray-800 shadow w-full rounded-lg divide-y divide-gray-200",
                    div {
                        class: "px-5 py-7",
                        label {
                            class: "font-semibold text-sm text-gray-600 dark:text-gray-50 pb-1 block",
                            "Username"
                        }
                        input {
                            class: "border rounded-lg px-3 py-2 mt-1 mb-5 text-sm w-full",
                            r#type: "text",
                        }
                        label {
                            class: "font-semibold text-sm text-gray-600 dark:text-gray-50 pb-1 block",
                            "E-mail"
                        }
                        input {
                            class: "border rounded-lg px-3 py-2 mt-1 mb-5 text-sm w-full",
                            r#type: "text",
                        }
                        label {
                            class: "font-semibold text-sm text-gray-600 dark:text-gray-50 pb-1 block",
                            "Password"
                        }
                        input {
                            class: "border rounded-lg px-3 py-2 mt-1 mb-5 text-sm w-full",
                            r#type: "password",
                        }
                        button {
                            class: "transition duration-200 bg-blue-500 hover:bg-blue-600 focus:bg-blue-700 focus:shadow-sm focus:ring-4 focus:ring-blue-500 focus:ring-opacity-50 text-white w-full py-2.5 rounded-lg text-sm shadow-sm hover:shadow-md font-semibold text-center inline-block",
                            r#type: "button",span {
                                class: "inline-block mr-2",
                                "Register"
                            }
                        }
                    }
                    div {
                        class: "py-5",
                        div {
                            class: "grid grid-cols-2 gap-1",
                            div {}
                            div {
                                class: "text-center sm:text-right whitespace-nowrap",
                                Link {
                                    class: "transition duration-200 mx-5 px-5 py-4 cursor-pointer font-normal text-sm rounded-lg text-gray-500 dark:text-white hover:bg-gray-100 dark:hover:text-black focus:outline-none focus:bg-gray-200 focus:ring-2 focus:ring-gray-400 focus:ring-opacity-50 ring-inset",
                                    to: "/login",
                                    span {
                                        class: "inline-block ml-1",
                                        "Sign In"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}
