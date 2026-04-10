use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TopBarProps {
    /// The title to display in the top bar.
    pub title: String,
    /// Whether to center the title.
    #[props(default = false)]
    pub center_title: bool,
    /// Optional element to display on the left (e.g., a back button or menu icon).
    pub leading: Option<Element>,
    /// Optional elements to display on the right (e.g., search, settings, profile).
    pub actions: Option<Vec<Element>>,
    /// Custom CSS class for the top bar container.
    #[props(default)]
    pub class: String,
    /// Custom CSS class for the title.
    #[props(default)]
    pub title_class: String,
    /// Custom CSS class for the leading element container.
    #[props(default)]
    pub leading_class: String,
    /// Custom CSS class for individual action items.
    #[props(default)]
    pub action_class: String,
}
