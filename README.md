# Androxus

Androxus is a highly customizable, mobile-first UI component library built specifically for the Dioxus framework. Designed with Android and Material Design 3 principles in mind, it provides robust and natively feeling components tailored for mobile application development in Rust.

## Features

* Material Design 3 Architecture: Built strictly adhering to native Android design patterns.
* 100% TailwindCSS Compatible: Every component is engineered to be fully overriding without specificity clashes. You can style the background, titles, active tabs, and icons purely using utility classes.
* Built-in SVG Icons: Out-of-the-box access to essential Material 3 vector icons optimized for Dioxus components.
* Dioxus Router Integration: The BottomTabBar seamlessly couples with Dioxus routers out of the box to track active routes natively.
* Zero-Friction Setup: No complex CSS overrides required. Under the hood, components utilize CSS layer logic to ensure your class attributes always have maximum precedence.

## Components Currently Available

* **BottomTabBar**: A resilient navigation bar with deep customization properties.
* **TopBar**: A highly configurable header/app bar supporting leading widgets and trailing action lists.
* Built-in **Icons** package (HomeIcon, SearchIcon, SettingsIcon, BackIcon, CloseIcon, CheckIcon, MenuIcon, BellIcon, HeartIcon, AddIcon)

## Installation

Run the following command in your project directory:

```bash
cargo add androxus
```

## Quick Start Examples

### 1. Setting up the framework
You can include the components and icons globally by importing the prelude module.

```rust
use androxus::prelude::*;
use dioxus::prelude::*;
```

### 2. Using the TopBar
The TopBar can take a simple title, or complex centered layouts with customizable leading elements and actions groups.

```rust
TopBar {
    title: "Application Settings".to_string(),
    center_title: true,
    class: "bg-surface text-on-surface",
    leading: Some(rsx! { 
        BackIcon { class: "w-6 h-6".to_string() } 
    }),
    actions: Some(vec![
        rsx! { SearchIcon { class: "w-6 h-6".to_string() } },
        rsx! { SettingsIcon { class: "w-6 h-6".to_string() } }
    ])
}
```

### 3. Integrated BottomTabBar
The BottomTabBar takes full advantage of Tailwind configuration and Dioxus routing. You can specifically target the active indicator background, active icon texts, and standard styling dynamically.

```rust
BottomTabBar {
    class: "bg-surface text-on-surface h-20",
    item_class: "text-gray-500",
    active_class: "font-bold",
    active_indicator_class: "bg-blue-200",
    active_icon_class: "text-blue-900",
    items: vec![
        BottomTabItem {
            icon: rsx! { HomeIcon { class: "w-6 h-6".to_string() } },
            title: "Home".to_string(),
            route: Route::HomePage {},
        },
        BottomTabItem {
            icon: rsx! { SearchIcon { class: "w-6 h-6".to_string() } },
            title: "Search".to_string(),
            route: Route::SearchPage {},
        },
        BottomTabItem {
            icon: rsx! { SettingsIcon { class: "w-6 h-6".to_string() } },
            title: "Settings".to_string(),
            route: Route::SettingPage {},
        },
    ]
}
```