#![allow(non_snake_case)]

mod components;
mod hooks;
mod pages;
mod mode;
mod data;

use data::model::PlayerBoxStatus;
use dioxus::prelude::*;
use dioxus_heroicons::{Icon, solid::Shape};
use mode::is_dark;
use dioxus_toast::{ToastFrame, ToastManager};

use crate::components::{navbar::NavBar, modal::PlayBox};

static DARK_MODE: dioxus::fermi::Atom<bool> = |_| {
    let dark = is_dark();
    mode::mode(dark);
    dark
};

static PLAYER_STATUS: dioxus::fermi::AtomRef<PlayerBoxStatus> = |_| { 
    PlayerBoxStatus {
        display: true,
        pause: true,
    }
};

static TOAST_MANAGER: dioxus::fermi::AtomRef<ToastManager> = |_| ToastManager::default();

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("TeacherPod Init");

    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {

    let toast = use_atom_ref(&cx, TOAST_MANAGER);

    cx.render(rsx! {
        ToastFrame {
            manager: toast,
            maximum: 8,
        }
        Router {
            NavBar {}
            PlayBox {}
            Route { to: "/", pages::discover::Discover {} }

            Route { to: "/login", pages::login::Login {} }
            Route { to: "/register", pages::login::Register {} }

            Route { to: "/user/:userid", pages::user::User {} }

            Route { to: "", pages::error::_404 {} }
            Footer {}
        }
    })
}

fn Footer(cx: Scope) -> Element {
    cx.render(rsx! {
        br {}
        div {
            class: "bg-gray-100 dark:bg-gray-600 pt-2",
            div {
                class: "flex pb-5 px-3 m-auto pt-5 border-t text-gray-800 text-sm flex-col md:flex-row max-w-6xl",
                div {
                    class: "mt-2 text-black dark:text-white",
                    "© Copyright 2022. All Rights Reserved."
                }
                div {
                    class: "md:flex-auto md:flex-row-reverse mt-2 flex-row flex",
                    a {
                        class: "w-6 mx-1 text-black dark:text-white",
                        href: "https://github.com/commune-org",
                        Icon {
                            icon: Shape::GlobeAlt
                        }
                    }
                }
            }
        }
    })
}