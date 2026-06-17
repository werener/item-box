use crate::core::models::*;
use dioxus::prelude::*;

// class: "bg-surface border border-surface-muted shadow-xl rounded-xl" - shadowed card
#[component]
pub fn App() -> Element {
    let _t: Tool = Tool::new(
        "Hammer".to_string(),
        "Bottom drawer".to_string(),
        500.0,
        None,
        vec![],
    );
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
