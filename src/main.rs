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
    let fragment: Signal<String> = use_signal(|| url_fragment());
    let mut new_fragment: Signal<String> = use_signal(|| " ".into());

    // use_effect is needed in case the element in question isn't rendered yet
    use_effect(move || {
        if !url_fragment().is_empty() {
            info!("Scrolling to fragment: '{}'", url_fragment());
            let document = window().unwrap().document().unwrap();
            if let Some(element) = document.get_element_by_id(&url_fragment()) {
                element.scroll_into_view();
            }
        }
    });

    if new_fragment() == url_fragment() {
        info!("Navigation happened already!");
    } else if new_fragment() == " " {
        info!("Went back (or loaded page).");
    } else {
        info!("Navigating to new fragment: '{}'", new_fragment());
        let url_fragment = new_fragment().clone();
        new_fragment.set(" ".into());
        navigator().push(Route::Page { url_fragment });
    }

    rsx! {
        p {
            Link { to: Route::Home {}, "Go Home" }
        }
        p { "Current fragment: '{fragment()}'" }

        div { class: "h-96" }
        Sec { title: "Section1", new_fragment }
        div { class: "h-96" }
        Sec { title: "Section2", new_fragment }
        div { class: "h-[300rem]" }
    }
}

#[component]
fn Sec(title: String, new_fragment: Signal<String>) -> Element {
    let id = title.to_lowercase().replace(' ', "-");
    rsx! {
        h1 { id: "{id}", onclick: move |_| new_fragment.set(id.clone()), "{title}" }
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
