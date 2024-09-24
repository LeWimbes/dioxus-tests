#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use std::rc::Rc;
use web_sys::{window, ScrollIntoViewOptions, ScrollToOptions};

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
    let scroll_to = |id: String| {
        if !id.is_empty() {
            let window = window().unwrap();
            let document = window.document().unwrap();

            if let Some(element) = document.get_element_by_id(&id) {
                let offset = if let Some(navbar) = document.get_element_by_id("navbar") {
                    let navbar_height = navbar.get_bounding_client_rect().height();
                    (navbar_height / 5.0) * 6.0
                } else {
                    96.0 // default value in case 1rem = 16px
                };

                let body_rect = document.body().unwrap().get_bounding_client_rect();
                let element_rect = element.get_bounding_client_rect();

                let element_position = element_rect.top() - body_rect.top();
                let offset_position = element_position - offset;

                let options = ScrollToOptions::new();
                options.set_top(offset_position);

                window.scroll_to_with_scroll_to_options(&options);
            }
        }
    };

    // use_effect is needed in case the element in question isn't rendered yet
    use_effect(move || {
        scroll_to(url_fragment());
    });

    let handler = NavigationHandler(Rc::new(move |new_fragment: String| {
        if new_fragment == url_fragment() {
            scroll_to(new_fragment);
        } else {
            navigator().replace(Route::Page { url_fragment: new_fragment });
        }
    }));

    use_context_provider(|| handler);

    rsx! {
        Navbar {}

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

pub fn Navbar() -> Element {
    rsx! {
        nav { class: "fixed top-0 z-50 px-8 w-full flex flex-col items-center bg-black shadow-md h-20",
            id: "navbar",
        }
        // take up space of navbar
        div { class: "h-20" }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Navbar {}

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
            Link { to: Route::from_route_segment("/page#section1").unwrap(), "Go to page section1 from route segment" }
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
