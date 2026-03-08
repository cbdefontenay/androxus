use dioxus::prelude::*;

const CSS: &str = include_str!("../styles/bottom_tabbar.css");

#[derive(Props, Clone, PartialEq)]
pub struct BottomTabBarProps<R: Routable + Clone + PartialEq + 'static> {
    /// The items to display in the tab bar.
    pub items: Vec<BottomTabItem<R>>,
    /// Custom CSS class for the tab bar container.
    #[props(optional)]
    pub class: Option<String>,
    /// Custom CSS class for the items.
    #[props(optional)]
    pub item_class: Option<String>,
    /// Custom CSS class for the active item.
    #[props(optional)]
    pub active_class: Option<String>,
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

/// A bottom tab bar component.
///
/// This component provides a Material 3 Navigation Bar that is fully customizable.
/// It automatically handles navigation using the Dioxus Router.
#[component]
pub fn BottomTabBar<R: Routable + Clone + PartialEq + 'static>(props: BottomTabBarProps<R>) -> Element {
    let route: R = use_route::<R>();

    rsx! {
        style { "{CSS}" }
        nav { class: format!("androxus-bottom-tabbar {}", props.class.unwrap_or_default()),
            for item in props.items {
                Link {
                    to: item.route.clone(),
                    class: format!(
                        "androxus-bottom-tabbar-item {} {}",
                        if route == item.route {
                            format!("active {}", props.active_class.clone().unwrap_or_default())
                        } else {
                            "".to_string()
                        },
                        props.item_class.clone().unwrap_or_default(),
                    ),
                    div { class: "androxus-bottom-tabbar-icon-container",
                        div { class: "androxus-bottom-tabbar-icon", {item.icon} }
                    }
                    span { class: "androxus-bottom-tabbar-title", "{item.title}" }
                }
            }
        }
    }
}
