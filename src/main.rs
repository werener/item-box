mod core;

use core::models::*;
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let t: Tool = Tool::new("Hammer".to_string(), "Bottom drawer".to_string(), 500.0, None, vec![]);
    rsx! {
        // metainfo
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        document::Stylesheet { href: asset!("/assets/main.css") }

        ToolDbg { t }
    }
}
#[component]
fn ToolDbg(t: Tool) -> Element {
    rsx!{
        div { class: "border-1 m-2 p-1 w-1/3 border-gray-400 text-white",
            h1 { "{t:#?}" }
        }
    }
}
