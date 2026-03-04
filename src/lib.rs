pub mod components;
pub mod prelude;

/*
Usage Example (Material Design 3):

use androxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    Home {},
    #[route("/search")]
    Search {},
    #[route("/settings")]
    Settings {},
}

#[component]
fn MainLayout() -> Element {
    rsx! {
        div {
            class: "main-container",
            Outlet::<Route> {}
            BottomTabBar {
                items: vec![
                    BottomTabItem {
                        icon: rsx! { HomeIcon {} },
                        title: "Home".to_string(),
                        route: Route::Home {},
                    },
                    BottomTabItem {
                        icon: rsx! { SearchIcon {} },
                        title: "Search".to_string(),
                        route: Route::Search {},
                    },
                    BottomTabItem {
                        icon: rsx! { SettingsIcon {} },
                        title: "Settings".to_string(),
                        route: Route::Settings {},
                    },
                ],
            }
        }
    }
}

// To display the component, simply include it in your layout.
// It follows Material Design 3 specifications with an active indicator pill.
*/