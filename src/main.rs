use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}


#[component]
fn App() -> Element {
    rsx! {
        div {
            id: "about",
            class: "flex flex-row",
            div {
                h1 {
                    "Ryan Hsiang" 
                }
                About {}
            }
            div {
                img {
                    class: "profile",
                    src: asset!("assets/profile.png"),
                    alt: "Profile picture", 
                }
                Info {}
            }
        }
        document::Stylesheet {
            href: asset!("assets/main.css"),
        }
        document::Stylesheet { href: asset!("assets/tailwind.css") }
    }
}

#[component]
fn About() -> Element {
    rsx! {
        div { class: "about",
            p {
                "I am Ryan Hsiang (項達均), a junior from National Taiwan University (NTU) majoring in Electrical Engineering.
                My research interests lie in machine learning and its applications to physics, mathematics, and engineering. In the summer of 2025, I will be participating in the Summer Undergraduate Research Fellowship (SURF) at Caltech,
                under the guidance of Prof. Anima Anandkumar."
            }

            p { class: "text-lg text-gray-700 leading-relaxed",
                "Outside of research, I have extensive experience in software development through various side projects and courses. 
                In particular, I am familiar with programming in Rust. In fact, this website is written in Rust."
            }
        }
    }
}

#[component]
fn Info() -> Element {
    rsx! {
        div {
            class: "info",
            div {
                class: "flex flex-row",
                id: "cv",
                a {
                    img {
                        src: asset!("assets/cv.png"),
                        alt: "cv",
                        style: "width: 25px;"
                    }
                }
                div { class: "sep" }
                a {
                    href: "https://github.com/durant42040",
                    img {
                        class: "github",
                        src: "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png",
                        alt: "GitHub",
                        style: "width: 25px;"
                    }
                }
            }
            a {
                href: "mailto:rhsiang@caltech.edu",
                class: "flex flex-row",
                id: "mail",
                img {
                    src: asset!("assets/mail.png"),
                    alt: "mail",
                    style: "width: 18px;"
                }
                p {
                    id: "mail-text",
                    "rhsiang@caltech.edu"
                }
            }
        }
    }
}