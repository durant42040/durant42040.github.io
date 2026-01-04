use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
struct ProjectProps {
    title: String,
    description: String,
    github_link: Option<String>,
    youtube_link: Option<String>,
    paper_link: Option<String>,
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
                github_link: Some("https://github.com/NTUEELightDance/LightDance-Editor".to_string()),
                youtube_link: Some("https://drive.google.com/file/d/13Mh13zdwgQuAHI1TSK-m7TzPZo8WxKoE/view?usp=sharing".to_string()),
                paper_link: None,
                image: asset!("assets/images/lightdance.png"),
            }
        }
    }
}

#[component]
pub fn Publications() -> Element {
    rsx! {
        div { id: "publications",
            h3 { id: "publications-title", "Publications" }
            Project {
                title: "LeanDojo-v2".to_string(),
                description: "LeanDojo-v2 is an end-to-end framework for training, evaluating, and deploying AI-assisted theorem provers for Lean 4. It combines repository tracing, lifelong dataset management, retrieval-augmented agents, Hugging Face fine-tuning, and external inference APIs into one toolkit.".to_string(),
                paper_link: Some("https://openreview.net/pdf?id=tnx1VvrcAn".to_string()),
                github_link: Some("https://github.com/lean-dojo/LeanDojo-v2".to_string()),
                youtube_link: None,
                image: asset!("assets/images/leandojo.png"),
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
                    if let Some(paper_link) = &props.paper_link {
                        a { href: "{paper_link}", class: "project-link",
                            img {
                                src: asset!("assets/icons/file.svg"),
                                alt: "Paper",
                                style: "width: 22px;",
                            }
                        }
                    }
                    if let Some(github_link) = &props.github_link {
                        a { href: "{github_link}", class: "project-link",
                            img {
                                src: asset!("assets/icons/github.png"),
                                alt: "GitHub",
                                style: "width: 22px;",
                            }
                        }
                    }
                    if let Some(youtube_link) = &props.youtube_link {
                        a { href: "{youtube_link}", class: "project-link",
                            img {
                                src: asset!("assets/icons/youtube.png"),
                                alt: "YouTube",
                                style: "width: 22px;",
                            }
                        }
                    }
                }
            }
        }
    }
}
