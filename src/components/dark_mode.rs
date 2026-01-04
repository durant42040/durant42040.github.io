use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

fn apply_dark_mode_class(is_dark: bool) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(html) = document.document_element() {
                if let Ok(html_element) = html.dyn_into::<HtmlElement>() {
                    let class_list = html_element.class_list();
                    if is_dark {
                        let _ = class_list.add_1("dark-mode");
                    } else {
                        let _ = class_list.remove_1("dark-mode");
                    }
                }
            }
        }
    }
}

fn is_night_time() -> bool {
    let date = js_sys::Date::new_0();
    let hour = date.get_hours();
    hour >= 18 || hour < 6
}

#[component]
pub fn DarkModeToggle() -> Element {
    use_effect(move || {
        let is_dark = is_night_time();
        apply_dark_mode_class(is_dark);
    });

    rsx! {}
}
