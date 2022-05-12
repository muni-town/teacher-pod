#[allow(non_snake_case)]
mod components;
mod hooks;
mod mode;

use components::navbar::NavBar;
use dioxus::prelude::*;
use mode::is_dark;

static DARK_MODE: dioxus::fermi::Atom<bool> = |_| is_dark();

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    
    let dark_mode = use_read(&cx, DARK_MODE);
    if *dark_mode {
        let _ = js_sys::eval("document.documentElement.classList.add('dark');");
    }

    cx.render(rsx! {
        NavBar {

        }
    })
}
