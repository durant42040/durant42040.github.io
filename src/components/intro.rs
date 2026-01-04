use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

fn is_dark_mode() -> bool {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(html) = document.document_element() {
                if let Ok(html_element) = html.dyn_into::<HtmlElement>() {
                    return html_element.class_list().contains("dark-mode");
                }
            }
        }
    }
    false
}

#[component]
pub fn Intro() -> Element {
    rsx! {
        div { id: "name",
            h1 { "Ryan Hsiang" }
            h2 { class: "name", "項達均" }
        }
        div { id: "about",
            div { id: "left",
                About {}
                Info {}
            }
            div { id: "right", Profile {} }
        }
    }
}

#[component]
fn About() -> Element {
    rsx! {
        div { class: "about",
            p {
                "I am Ryan Hsiang, a senior from "
                a {
                    style: "text-decoration: none;",
                    href: "https://www.ntu.edu.tw/",
                    "National Taiwan University"
                }
                " (NTU) majoring in Electrical Engineering.
                My research interests lie in machine learning and its applications to physics, mathematics, and engineering. In the summer of 2025, I participated in the "
                a {
                    style: "text-decoration: none;",
                    href: "https://sfp.caltech.edu/undergraduate-research/programs/surf",
                    "Summer Undergraduate Research Fellowship"
                }
                " (SURF) at "
                a {
                    style: "text-decoration: none;",
                    href: "https://www.caltech.edu/",
                    "Caltech"
                }
                ",
                under the guidance of Prof. "
                a {
                    style: "text-decoration: none;",
                    href: "https://tensorlab.cms.caltech.edu/users/anima/index.html",
                    "Anima Anandkumar"
                }
                " and Dr. "
                a {
                    style: "text-decoration: none;",
                    href: "https://www.robertj1.com",
                    "Robert Joseph George"
                }
                "."
            }

            p { class: "text-lg text-gray-700 leading-relaxed",
                "Outside of research, I have extensive experience in software development through various side projects and courses.
                In particular, I led a group of students at NTUEE to develop the "
                a {
                    style: "text-decoration: none;",
                    href: "https://github.com/NTUEELightDance/LightDance-Editor",
                    "LightDance Editor"
                }
                ", a full-stack web application for light and dance choreography built with Rust, Blender, and MySQL."
            }
        }
    }
}

#[component]
fn Info() -> Element {
    let mut dark_mode = use_signal(|| false);
    
    use_effect(move || {
        dark_mode.set(is_dark_mode());
    });
    
    rsx! {
        div { class: "info",
            a { id: "cv", href: "/cv.pdf",
                img {
                    src: if dark_mode() { asset!("assets/icons/cv-light.png") } else { asset!("assets/icons/cv.png") },
                    alt: "cv",
                    style: "width: 22px;",
                }
            }
            p { class: "line", "/" }
            a { href: "https://github.com/durant42040",
                img {
                    class: "github",
                    src: if dark_mode() { asset!("assets/icons/github-light.png") } else { asset!("assets/icons/github.png") },
                    alt: "GitHub",
                    style: "width: 18px;",
                }
            }
            p { class: "line", "/" }
            a { href: "https://www.linkedin.com/in/ryan-hsiang",
                img {
                    class: "linkedin",
                    src: if dark_mode() { asset!("assets/icons/linkedin-light.png") } else { asset!("assets/icons/linkedin.png") },
                    alt: "LinkedIn",
                    style: "width: 18px;",
                }
            }
            p { class: "line", "/" }
            a {
                id: "mail",
                href: "mailto:b11901040@ntu.edu.tw",
                style: "text-decoration: none;",
                img {
                    id: "mail-icon",
                    src: if dark_mode() { asset!("assets/icons/mail-light.png") } else { asset!("assets/icons/mail.png") },
                    alt: "mail",
                    style: "width: 18px;",
                }
                p { id: "mail-text", "b11901040@ntu.edu.tw" }
            }
        }
    }
}

#[component]
fn Profile() -> Element {
    rsx! {
        img {
            class: "profile",
            src: asset!("assets/images/profile.png"),
            alt: "Profile picture",
        }
    }
}
