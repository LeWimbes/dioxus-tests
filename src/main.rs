#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use std::rc::Rc;
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

pub struct NavigationHandler(Rc<dyn Fn(String)>);

impl Clone for NavigationHandler {
    fn clone(&self) -> Self {
        NavigationHandler(Rc::clone(&self.0))
    }
}

impl NavigationHandler {
    pub fn call(&self, new_fragment: String) {
        self.0(new_fragment)
    }
}

#[component]
fn Page(url_fragment: ReadOnlySignal<String>) -> Element {

    // use_effect is needed in case the element in question isn't rendered yet
    use_effect(move || {
        if !url_fragment().is_empty() {
            let document = window().unwrap().document().unwrap();
            if let Some(element) = document.get_element_by_id(&url_fragment()) {
                element.scroll_into_view();
            }
        }
    });

    let handler = NavigationHandler(Rc::new(move |new_fragment: String| {
        if new_fragment != url_fragment() {
            navigator().replace(Route::Page { url_fragment: new_fragment });
        }
    }));

    use_context_provider(|| handler);

    rsx! {
        p {
            Link { to: Route::Home {}, "Go Home" }
        }

        div { class: "h-96" }
        Sec { title: "Section1" }
        div { class: "h-96" }
        Sec { title: "Section2" }
        div { class: "h-[300rem]" }
    }
}

#[component]
fn Sec(title: String) -> Element {
    let handler: NavigationHandler = use_context();

    let id = title.to_lowercase().replace(' ', "-");
    rsx! {
        h1 { id: "{id}", onclick: move |_| handler.call(id.clone()), "{title}" }
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
