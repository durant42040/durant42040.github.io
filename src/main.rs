use dioxus::prelude::*;
mod components;

#[derive(Routable, Clone)]
enum Route {
    // The home page is at the / route
    #[route("/")]
    Home {},
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "container",
            components::Intro {}
            components::Projects {}
        }
        document::Stylesheet { href: asset!("assets/main.css") }
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
