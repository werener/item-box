// src/ui/components/icons.rs
use dioxus::prelude::*;

/// Custom icon of a filter/funnel (8x8).
#[component]
pub fn FunnelIcon(#[props(extends = SvgAttributes)] attributes: Vec<Attribute>) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 8 8",
            width: "100%",
            height: "100%",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            ..attributes,
            polygon { points: "7.33 1 0.67 1 3.33 4.15 3.33 6.33 4.67 7 4.67 4.15 7.33 1" }
        }
    }
}

/// Custom icon of a pencil (8x8).
#[component]
pub fn PencilIcon(#[props(extends = SvgAttributes)] attributes: Vec<Attribute>) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            width: "100%",
            height: "100%",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            ..attributes,
            // Main body and tip of the pencil
            path { d: "M4 20h4l10.5 -10.5a2.828 2.828 0 1 0 -4 -4l-10.5 10.5v4" }
            // Line separating the sharpened wood from the main body
            path { d: "M13.5 6.5l3 3" }
        }
    }
}

/// Custom icon of a trash can (8x8).
#[component]
pub fn TrashIcon(#[props(extends = SvgAttributes)] attributes: Vec<Attribute>) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            width: "100%",
            height: "100%",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            ..attributes,
            // The lid
            polyline { points: "3 6 5 6 21 6" }
            // The body and the handle
            path { d: "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" }
            // Vertical lines inside the body
            line {
                x1: "10",
                y1: "11",
                x2: "10",
                y2: "17",
            }
            line {
                x1: "14",
                y1: "11",
                x2: "14",
                y2: "17",
            }
        }
    }
}
