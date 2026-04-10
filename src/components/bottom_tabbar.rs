use crate::code::bottom_tabbar::BottomTabBarProps;
use dioxus::prelude::*;

const CSS: &str = include_str!("../styles/bottom_tabbar.css");

/// A bottom tab bar component.
///
/// This component provides a Material 3 Navigation Bar that is fully customizable.
/// It automatically handles navigation using the Dioxus Router.
#[component]
pub fn BottomTabBar<R: Routable + Clone + PartialEq + 'static>(props: BottomTabBarProps<R>) -> Element {
    let route = use_route::<R>();

    rsx! {
        style { "{CSS}" }
        nav { class: "androxus-bottom-tabbar {props.class}",
            for item in props.items {
                Link {
                    to: item.route.clone(),
                    class: format!(
                        "androxus-bottom-tabbar-item {} {}",
                        props.item_class,
                        if route == item.route {
                            format!("active {}", props.active_class)
                        } else {
                            String::new()
                        }
                    ),
                    div { class: format!(
                            "androxus-bottom-tabbar-icon-container {} {}",
                            props.indicator_class,
                            if route == item.route { props.active_indicator_class.clone() } else { String::new() }
                        ),
                        div { class: format!(
                                "androxus-bottom-tabbar-icon {} {}",
                                props.icon_class,
                                if route == item.route { props.active_icon_class.clone() } else { String::new() }
                            ), {item.icon} }
                    }
                    span { class: format!(
                            "androxus-bottom-tabbar-title {} {}",
                            props.title_class,
                            if route == item.route { props.active_title_class.clone() } else { String::new() }
                        ), "{item.title}" }
                }
            }
        }
    }
}
