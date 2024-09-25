#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        nav {
            class: "fixed top-0 z-50 px-8 w-full flex flex-col items-center bg-black shadow-md h-20",
            id: "navbar",
        }
        // take up space of navbar
        div { class: "h-20" }

        div { class: "bg-blue-200 h-64" } // header

        div { class: "relative", // body

            div { class: "w-48 sticky top-24 hidden lg:block m-4", // side navbar
                ul { class: "space-y-4 bg-gray-100 p-4 h-96" }
            }

            div { class: "px-8 w-full flex flex-col items-center", // main content
                div { class: "w-full max-w-3xl",
                    p {
                        Link { to: Route::Home {}, "Go Home" }
                    }

                    div { class: "h-96" }
                    h1 {  "Section1" }
                    div { class: "h-96" }
                    h1 {  "Section2" }
                    div { class: "h-[300rem]" }
                }
            }
        }

        div { class: "h-[100rem] bg-gray-500 " } // footer
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        p {
            Link { to: "https://dioxuslabs.com/", new_tab: true, "dioxuslabs.com" }
        }
        p {
            Link { to: Route::Blog { id: count() }, "Go to blog" }
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
