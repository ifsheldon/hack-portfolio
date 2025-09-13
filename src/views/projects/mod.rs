use crate::components::GithubRepoCard;
use crate::personal_info::{PROJECTS, PROJECTS_HEADER};
use dioxus::prelude::*;
const PROJECTS_CSS: Asset = asset!("/src/views/projects/projects.css");
#[component]
pub fn Projects() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: PROJECTS_CSS,
        }
        div {
            class: "projects-container",
            h1 {

                "{PROJECTS_HEADER.title}"
            }
            p {

                "{PROJECTS_HEADER.description}"
            }
            div {
                class: "projects-grid",
                for project in &PROJECTS.data {
                    GithubRepoCard {
                        name: project.name.to_string(),
                        description: project.description.to_string(),
                        url: project.url.to_string(),
                    }
                }
            }
        }
    }
}
