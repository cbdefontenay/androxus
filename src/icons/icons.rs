use dioxus::prelude::*;

#[component]
pub fn PasswordIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-6 h-6".to_string());

    rsx! {
        svg {
            class,
            view_box: "0 0 24 24",
            fill: "currentColor",
            xmlns: "http://www.w3.org/2000/svg",
            path { d: "M18 10h-1.2V7.4c0-2.6-2.2-4.8-4.8-4.8S7.2 4.8 7.2 7.4V10H6c-1.1 0-2 .9-2 2v8c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2v-8c0-1.1-.9-2-2-2m-10.8 0V7.4c0-2.1 1.7-3.6 3.6-3.6s3.6 1.5 3.6 3.6V10zm5.4 6.1c0 .1-.1.3-.2.4l.6 1.5h-2l.6-1.5c-.1-.1-.2-.3-.2-.4 0-.4.3-.8.8-.8s.8.4.8.8" }
        }
    }
}

#[component]
pub fn HistoryIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-6 h-6".to_string());

    rsx! {
        svg {
            class,
            view_box: "0 0 24 24",
            fill: "currentColor",
            xmlns: "http://www.w3.org/2000/svg",
            path { d: "M13 3c-4.97 0-9 4.03-9 9H1l3.89 3.89 4.11-3.89H6c0-3.87 3.13-7 7-7s7 3.13 7 7-3.13 7-7 7c-1.93 0-3.68-.79-4.94-2.06l-1.42 1.42C8.27 19.99 10.51 21 13 21c4.97 0 9-4.03 9-9s-4.03-9-9-9m-1 5v5l4.28 2.54.72-1.21-3.5-2.08V8z" }
        }
    }
}

#[component]
pub fn SettingsIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-6 h-6".to_string());

    rsx! {
        svg {
            class,
            view_box: "0 0 24 24",
            fill: "currentColor",
            xmlns: "http://www.w3.org/2000/svg",
            path { d: "m19.4 13c0-.3.1-.6.1-.9s0-.6-.1-.9l2.1-1.7c.2-.2.2-.4.1-.6l-2-3.5c-.1-.2-.4-.3-.6-.2l-2.5 1c-.5-.4-1.1-.7-1.7-1l-.4-2.6c0-.2-.2-.4-.5-.4h-4c-.3 0-.5.2-.5.4l-.4 2.7c-.6.3-1.1.6-1.7 1l-2.5-1c-.2-.1-.5 0-.6.2l-2 3.5c-.1.2-.1.4.1.6l2.1 1.7c0 .3-.1.6-.1.9s0 .6.1.9l-2.1 1.7c-.2.2-.2.4-.1.6l2 3.5c.1.2.4.3.6.2l2.5-1c.5.4 1.1.7 1.7 1l.4 2.6c0 .2.2.4.5.4h4c.3 0 .5-.2.5-.4l.4-2.7c.6-.3 1.1-.6 1.7-1l2.5 1c.2.1.5 0 .6-.2l2-3.5c.1-.2.1-.4-.1-.6l-2.1-1.7m-7.4 2.5c-1.9 0-3.5-1.6-3.5-3.5s1.6-3.5 3.5-3.5 3.5 1.6 3.5 3.5-1.6 3.5-3.5 3.5" }
        }
    }
}

#[component]
pub fn LoadingIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "".to_string());

    rsx! {
        svg {
            class: "{class}",
            style: "width: 100%; height: 100%;",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            circle {
                style: "opacity: 0.25;",
                cx: "12",
                cy: "12",
                r: "10",
                stroke: "currentColor",
                stroke_width: "4",
            }
            path {
                style: "opacity: 0.75;",
                fill: "currentColor",
                d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z",
            }
        }
    }
}

#[component]
pub fn HomeIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-6 h-6".to_string());
    rsx! {
        svg { class, view_box: "0 0 24 24", fill: "currentColor", xmlns: "http://www.w3.org/2000/svg",
            path { d: "M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z" }
        }
    }
}

#[component]
pub fn SearchIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-6 h-6".to_string());
    rsx! {
        svg { class, view_box: "0 0 24 24", fill: "currentColor", xmlns: "http://www.w3.org/2000/svg",
            path { d: "M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z" }
        }
    }
}

#[component]
pub fn BackIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-6 h-6".to_string());
    rsx! {
        svg { class, view_box: "0 0 24 24", fill: "currentColor", xmlns: "http://www.w3.org/2000/svg",
            path { d: "M20 11H7.83l5.59-5.59L12 4l-8 8 8 8 1.41-1.41L7.83 13H20v-2z" }
        }
    }
}

#[component]
pub fn CloseIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-6 h-6".to_string());
    rsx! {
        svg { class, view_box: "0 0 24 24", fill: "currentColor", xmlns: "http://www.w3.org/2000/svg",
            path { d: "M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z" }
        }
    }
}

#[component]
pub fn CheckIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-6 h-6".to_string());
    rsx! {
        svg { class, view_box: "0 0 24 24", fill: "currentColor", xmlns: "http://www.w3.org/2000/svg",
            path { d: "M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z" }
        }
    }
}

#[component]
pub fn MenuIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-6 h-6".to_string());
    rsx! {
        svg { class, view_box: "0 0 24 24", fill: "currentColor", xmlns: "http://www.w3.org/2000/svg",
            path { d: "M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z" }
        }
    }
}

#[component]
pub fn BellIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-6 h-6".to_string());
    rsx! {
        svg { class, view_box: "0 0 24 24", fill: "currentColor", xmlns: "http://www.w3.org/2000/svg",
            path { d: "M12 22c1.1 0 2-.9 2-2h-4c0 1.1.9 2 2 2zm6-6v-5c0-3.07-1.63-5.64-4.5-6.32V4c0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5v.68C7.64 5.36 6 7.92 6 11v5l-2 2v1h16v-1l-2-2zm-2 1H8v-6c0-2.48 1.51-4.5 4-4.5s4 2.02 4 4.5v6z" }
        }
    }
}

#[component]
pub fn HeartIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-6 h-6".to_string());
    rsx! {
        svg { class, view_box: "0 0 24 24", fill: "currentColor", xmlns: "http://www.w3.org/2000/svg",
            path { d: "M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z" }
        }
    }
}

#[component]
pub fn AddIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-6 h-6".to_string());
    rsx! {
        svg { class, view_box: "0 0 24 24", fill: "currentColor", xmlns: "http://www.w3.org/2000/svg",
            path { d: "M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z" }
        }
    }
}
