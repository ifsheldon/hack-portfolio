use dioxus::prelude::*;
use crate::components::ExperienceCard;
use crate::personal_info::EXPERIENCE;
const CAREER_CSS: Asset = asset!("/src/views/career/career.css");
#[component]
pub fn Career() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: CAREER_CSS,
        }
        div {
            class: "experience-container",
            h1 {

                "{EXPERIENCE.title}"
            }
            h3 {

                "{EXPERIENCE.subtitle}"
            }
            p {

                "{EXPERIENCE.description}"
            }
            for section in EXPERIENCE.sections.iter() {
                div {
                    class: "experience-section",
                    h2 {

                        "{section.title}"
                    }
                    for exp in section.experiences.iter() {
                        ExperienceCard {
                            experience: exp.clone(),
                        }
                    }
                }
            }
        }
    }
}
