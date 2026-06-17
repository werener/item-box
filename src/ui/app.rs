use crate::core::models::*;
use dioxus::prelude::*;

// class: "bg-surface border border-surface-muted shadow-xl rounded-xl" - shadowed card
#[component]
pub fn App() -> Element {
    rsx! {
        // metainfo
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        // ToolDbg { t }
        div { class: "w-screen h-screen bg-surface" }
    }
}
#[component]
fn ToolDbg(t: Tool) -> Element {
    rsx! {
        div { class: "border-1 m-2 p-1 w-1/3",
            h1 { "{t:#?}" }
        }
    }
}
