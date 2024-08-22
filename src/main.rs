#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    launch(App);
}

fn App() -> Element {
    rsx! {
        div {
            class: "flex flex-col",
            div {
                "Different Fonts:"
            }
            div {
                class: "font-inter",
                "Inter"
            }
            div {
                class: "font-inter font-bold",
                "Inter Bold"
            }
            div {
                class: "font-inter font-bold italic",
                "Inter Bold Italic"
            }
            div {
                class: "font-faBrands font-normal",
                ""
            }
            div {
                class: "font-faRegular font-normal",
                ""
            }
            div {
                class: "font-faSolid font-black",
                ""
            }
        }
    }
}
