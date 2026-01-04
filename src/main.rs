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
        link { rel: "stylesheet", href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" }
        div {
            id: "overlay",
            style: "background-color:#FFFFFF; position:absolute; top:0px; left:0px; width:100%; height:100%; z-index:2000;",
        }
        components::DarkModeToggle {}
        div { class: "container",
            components::Intro {}
            components::Publications {}
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
