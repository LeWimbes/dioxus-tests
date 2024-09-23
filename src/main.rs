#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use web_sys::window;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/page#:url_fragment")]
    Page { url_fragment: String },
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
fn Page(url_fragment: ReadOnlySignal<String>) -> Element {
    let mut fragment: Signal<String> = use_signal(|| url_fragment());

    use_effect(move || {
        let document = window().unwrap().document().unwrap();
        if let Some(element) = document.get_element_by_id(&fragment()) {
            element.scroll_into_view_with_bool(true);
        }
    });

    let mut update_fragment = move |new_fragment: String| {
        if new_fragment != url_fragment() {
            navigator().replace(Route::Page { url_fragment: new_fragment.clone() });
            fragment.set(new_fragment);
        }
    };

    rsx! {
        p {
            Link { to: Route::Home {}, "Go Home" }
        }
        p { "Current fragment: '{fragment()}'" }

        div { class: "h-96" }
        h1 { id: "section1", onclick: move |_| update_fragment("section1".into()), "Section1" }
        div { class: "h-96" }
        h1 { id: "section2", onclick: move |_| update_fragment("section2".into()), "Section2" }
        div { class: "h-[300rem]" }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        p {
            Link {
                to: Route::Page {
                    url_fragment: "".into(),
                },
                "Go to page"
            }
        }
        p {
            Link { to: Route::from_route_segment("/page").unwrap(), "Go to page from route segment" }
        }
        p {
            Link {
                to: Route::Page {
                    url_fragment: "section1".into(),
                },
                "Go to page section1"
            }
        }
        p {
            Link {
                to: Route::Page {
                    url_fragment: "section2".into(),
                },
                "Go to page section2"
            }
        }
    }
}
