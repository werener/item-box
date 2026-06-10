mod core;

use core::models::*;
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let t: Tool = Tool {
        name: "Hammer".to_string(),
        cost: 500,
    };
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
        div { class: "border-1 m-2 p-1 w-1/6 border-gray-400 text-white",
            h1 { "{t.name}" }
            h3 { "{t.cost}" }
        }
    }
}
