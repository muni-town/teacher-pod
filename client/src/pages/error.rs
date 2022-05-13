use dioxus::prelude::*;

pub fn _404(cx: Scope) -> Element {
    cx.render(rsx! {
        main {
            class: "h-180 w-full flex flex-col justify-center items-center",
            h1 {
                class: "text-9xl font-extrabold text-black dark:text-white tracking-widest",
                "404"
            }
            div {
                class: "bg-[#FF6A3D] px-2 text-black dark:text-white text-sm rounded rotate-12 absolute",
                "Page Not Found"
            }
            button {
                class: "mt-5",
                a {
                    class: "relative inline-block text-sm font-medium text-[#FF6A3D] group active:text-orange-500 focus:outline-none focus:ring",
                    span {
                        class: "absolute inset-0 transition-transform translate-x-0.5 translate-y-0.5 bg-[#FF6A3D] group-hover:translate-y-0 group-hover:translate-x-0",
                        
                    }
                    span {
                        class: "relative block px-8 py-3 bg-gray-100 dark:bg-gray-700 border border-current",
                        Link {
                            to: "/",
                            "Home"
                        }
                    }
                }
            }
        }
    })
}