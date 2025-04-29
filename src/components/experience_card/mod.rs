use crate::data::ExperienceItem;
use dioxus::prelude::*;
const EXPERIENCE_CARD_CSS: Asset = asset!(
    "/src/components/experience_card/experience_card.css"
);
#[component]
pub fn ExperienceCard(experience: ExperienceItem) -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: EXPERIENCE_CARD_CSS,
        }
        div {
            class: "experience-card",
            div {
                class: "experience-card-logo-div",
                img {
                    src: "{experience.logo}",
                    class: "experience-card-logo",
                    alt: "{experience.company}",
                }
            }
            div {
                class: "experience-card-body-div",
                div {
                    class: "experience-card-header-div",
                    div {
                        class: "experience-card-heading-left",
                        h3 {
                            class: "experience-card-title",
                            "{experience.title}"
                        }
                        p {
                            class: "experience-card-company",
                            a {
                                href: "{experience.company_url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "{experience.company}"
                            }
                        }
                    }
                    div {
                        class: "experience-card-heading-right",
                        p {
                            class: "experience-card-duration",
                            "{experience.duration}"
                        }
                        p {
                            class: "experience-card-location",
                            "{experience.location}"
                        }
                    }
                }
                p {
                    class: "experience-card-description",
                    "{experience.description}"
                }
            }
        }
    }
}
