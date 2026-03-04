use dioxus::prelude::*;

const CSS: &str = include_str!("../styles/topbar.css");

#[derive(Props, Clone, PartialEq)]
pub struct TopBarProps {
    /// The title to display in the top bar.
    pub title: String,
    /// Whether to center the title.
    #[props(default = false)]
    pub center_title: bool,
    /// Optional element to display on the left (e.g., a back button or menu icon).
    #[props(optional)]
    pub leading: Option<Element>,
    /// Optional elements to display on the right (e.g., search, settings, profile).
    #[props(optional)]
    pub actions: Option<Vec<Element>>,
    /// Custom CSS class for the top bar container.
    #[props(optional)]
    pub class: Option<String>,
    /// Custom CSS class for the title.
    #[props(optional)]
    pub title_class: Option<String>,
}

/// A top app bar component.
///
/// This component provides a Material 3 Top App Bar that is fully customizable.
#[component]
pub fn TopBar(props: TopBarProps) -> Element {
    rsx! {
        style { "{CSS}" }
        header {
            class: format!("androxus-topbar {}", props.class.unwrap_or_default()),
            
            if let Some(leading) = props.leading {
                div { class: "androxus-topbar-leading", {leading} }
            }
            
            div { 
                class: format!(
                    "androxus-topbar-title-container {}", 
                    if props.center_title { "center-title" } else { "" }
                ),
                h1 { 
                    class: format!("androxus-topbar-title {}", props.title_class.unwrap_or_default()),
                    "{props.title}" 
                }
            }
            
            if let Some(actions) = props.actions {
                div { class: "androxus-topbar-actions",
                    for action in actions {
                        div { class: "androxus-topbar-action-item", {action} }
                    }
                }
            }
        }
    }
}
