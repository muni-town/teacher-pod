#[allow(non_snake_case)]

mod components;
mod hooks;
mod pages;
mod mode;
mod data;

use dioxus::prelude::*;
use dioxus_heroicons::{Icon, solid::Shape};
use mode::is_dark;

use crate::components::{navbar::NavBar, modal::PlayBox};

static DARK_MODE: dioxus::fermi::Atom<bool> = |_| {
    let dark = is_dark();
    mode::mode(dark);
    dark
};

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            NavBar {}
            PlayBox {}
            Route { to: "/", pages::discover::Discover {} }
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