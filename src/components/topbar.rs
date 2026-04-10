use crate::code::topbar::TopBarProps;
use dioxus::prelude::*;

const CSS: &str = include_str!("../styles/topbar.css");

/// A top app bar component.
///
/// This component provides a Material 3 Top App Bar that is fully customizable.
#[component]
pub fn TopBar(props: TopBarProps) -> Element {
    let center_class = if props.center_title { "center-title" } else { "" };
    rsx! {
        style { "{CSS}" }
        header { class: "androxus-topbar {props.class}",
            if let Some(leading) = props.leading {
                div { class: "androxus-topbar-leading {props.leading_class}", {leading} }
            }
            div {
                class: "androxus-topbar-title-container {center_class}",
                h1 { class: "androxus-topbar-title {props.title_class}",
                    "{props.title}"
                }
            }
            if let Some(actions) = props.actions {
                div { class: "androxus-topbar-actions",
                    for action in actions {
                        div { class: "androxus-topbar-action-item {props.action_class}", {action} }
                    }
                }
            }
        }
    }
}
