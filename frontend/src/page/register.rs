#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Register(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            h1 { "Register" }
            p { "This is the register page." }
        }
    })
}
