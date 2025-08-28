use dioxus::prelude::*;

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
                "I am Ryan Hsiang, a junior from "
                a {
                    style: "text-decoration: none;",
                    href: "https://www.ntu.edu.tw/",
                    "National Taiwan University"
                }
                ", (NTU) majoring in Electrical Engineering.
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
    rsx! {
        div { class: "info",
            a { id: "cv", href: "/cv.pdf",
                img {
                    src: asset!("assets/icons/cv.png"),
                    alt: "cv",
                    style: "width: 22px;",
                }
            }
            p { class: "line", "/" }
            a { href: "https://github.com/durant42040",
                img {
                    class: "github",
                    src: asset!("assets/icons/github.png"),
                    alt: "GitHub",
                    style: "width: 18px;",
                }
            }
            p { class: "line", "/" }
            a { href: "https://www.linkedin.com/in/ryan-hsiang-1a211a2a9",
                img {
                    class: "linkedin",
                    src: asset!("assets/icons/linkedin.png"),
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
                    src: asset!("assets/icons/mail.png"),
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
