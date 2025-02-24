use dioxus::prelude::*;

use crate::components::hero::Hero;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "splash",
            Hero {}
        }
    }
}
