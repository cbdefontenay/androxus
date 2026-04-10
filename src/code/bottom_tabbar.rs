use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BottomTabBarProps<R: Routable + Clone + PartialEq + 'static> {
    /// The items to display in the tab bar.
    pub items: Vec<BottomTabItem<R>>,
    /// Custom CSS class for the tab bar container.
    #[props(default)]
    pub class: String,
    /// Custom CSS class for the items.
    #[props(default)]
    pub item_class: String,
    /// Custom CSS class for the active item.
    #[props(default)]
    pub active_class: String,
    /// Custom CSS class for the icon container (indicator).
    #[props(default)]
    pub indicator_class: String,
    /// Custom CSS class for the icon container (indicator) when active.
    #[props(default)]
    pub active_indicator_class: String,
    /// Custom CSS class for the icon.
    #[props(default)]
    pub icon_class: String,
    /// Custom CSS class for the icon when active.
    #[props(default)]
    pub active_icon_class: String,
    /// Custom CSS class for the title text.
    #[props(default)]
    pub title_class: String,
    /// Custom CSS class for the title text when active.
    #[props(default)]
    pub active_title_class: String,
}

#[derive(Clone)]
pub struct BottomTabItem<R: Routable + Clone + PartialEq + 'static> {
    /// The icon to display. This can be any Element (e.g., an svg! or rsx!).
    pub icon: Element,
    /// The label to display under the icon.
    pub title: String,
    /// The route to navigate to when the item is clicked.
    pub route: R,
}

impl<R: Routable + Clone + PartialEq + 'static> PartialEq for BottomTabItem<R> {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.route == other.route
    }
}
