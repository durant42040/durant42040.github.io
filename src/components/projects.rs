use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
struct ProjectProps {
    title: String,
    description: String,
    github_link: String,
    youtube_link: String,
    image: Asset,
}

#[component]
pub fn Projects() -> Element {
    rsx! {
        div { id: "projects",
            h3 { id: "projects-title", "Projects" }
            Project {
                title: "NTUEE Light Dance".to_string(),
                description: "Developed a full-stack web application using Rust, Blender, and MySQL to design and
                                                                                        simulate light patterns and effects for 9 dancers and 18 props. Designed LED- and fiber-embedded 
                                                                                        costumes for each dancer, controlled via Raspberry Pi devices connected to a WebSocket server for 
                                                                                        real-time synchronization. Implemented dynamic moving light effects, such as a spinning orb and 
                                                                                        a large energy pulse, using JavaScript."
                    .to_string(),
                github_link: "https://github.com/NTUEELightDance/LightDance-Editor".to_string(),
                youtube_link: "https://drive.google.com/file/d/13Mh13zdwgQuAHI1TSK-m7TzPZo8WxKoE/view?usp=sharing"
                    .to_string(),
                image: asset!("assets/images/lightdance.png"),
            }
        }
    }
}

#[component]
fn Project(props: ProjectProps) -> Element {
    rsx! {
        div { class: "project",
            img {
                src: "{props.image}",
                alt: "Project Image",
                class: "project-image",
            }
            div { class: "project-info",
                h4 { "{props.title}" }
                p { class: "description", "{props.description}" }
                div { class: "links",
                    a { href: "{props.github_link}", class: "project-link",
                        img {
                            src: asset!("assets/icons/github.png"),
                            alt: "GitHub",
                            style: "width: 22px;",
                        }
                    }
                    a { href: "{props.youtube_link}", class: "project-link",
                        img {
                            src: asset!("assets/icons/youtube.png"),
                            alt: "GitHub",
                            style: "width: 22px;",
                        }
                    }
                }
            }
        }
    }
}
