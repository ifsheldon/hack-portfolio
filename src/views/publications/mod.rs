use crate::components::PublicationCard;
use crate::personal_info::PUBLICATIONS;
use dioxus::prelude::*;
const PUBLICATIONS_CSS: Asset = asset!("/src/views/publications/publications.css");
#[component]
pub fn Publications() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: PUBLICATIONS_CSS,
        }
        div {
            class: "publications-container",
            p {
                class: "publications-description",
                "{PUBLICATIONS.description}"
            }
            for section in PUBLICATIONS.sections.iter() {
                div {
                    class: "publications-section",
                    h2 {

                        "{section.category}"
                    }
                    for item in section.publications.iter() {
                        PublicationCard {
                            publication: item.clone(),
                        }
                    }
                }
            }
        }
    }
}
