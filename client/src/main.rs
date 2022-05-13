#[allow(non_snake_case)]
mod components;
mod hooks;
mod pages;
mod mode;
mod data;

use dioxus::prelude::*;
use mode::is_dark;

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
            Route { to: "/", pages::discover::Discover {} }
            Route { to: "", h1 { "404 Not Found" } }
        }
    })
}