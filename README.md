# Androxus

**Androxus** is a UI component based library for the Dioxus framework for ***Android*** only.

#### Current components:

- [x] Bottom tab bar.
- [x] Top bar.  

This crate is still young, but the components listed above are pretty solid, at least to my needs.

## Installation:
```bash
cargo add androxus
```

### Example:

```rust
use androxus::prelude::*;

BottomTabBar {
    items: vec![
        BottomTabItem {
            icon: rsx! {
                PasswordIcon { class : "w-6 h-6" }
            },
            title: "Home".to_string(),
            route: Route::HomePage {},
        },
        BottomTabItem {
            icon: rsx! {
                HistoryIcon { class : "w-6 h-6" }
            },
            title: "Second".to_string(),
            route: Route::SearchPage {},
        },
        BottomTabItem {
            icon: rsx! {
                SettingsIcon { class : "w-6 h-6" }
            },
            title: "Settings".to_string(),
            route: Route::SettingPage {},
        },
    ], 
}
```