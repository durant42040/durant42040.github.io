use dioxus::prelude::*;
mod components;

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home {},
}

#[component]
fn Home() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("assets/main.css") }
        div { class: "container",
            components::Intro {}
            components::Projects {}
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

fn main() {
    dioxus::launch(App);
}
