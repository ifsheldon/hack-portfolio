use crate::data::Degree;
use dioxus::prelude::*;
const DEGREE_CARD_CSS: Asset = asset!("/src/components/degree_card/degree_card.css");
#[component]
pub fn DegreeCard(degree: Degree) -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: DEGREE_CARD_CSS,
        }
        div {
            class: "degree-card",
            if let Some(logo) = degree.logo {
                div {
                    class: "card-img",
                    img {
                        style: "max-width: 100%; max-height: 100%; transform: scale(0.9);",
                        src: "{logo}",
                        alt: "{degree.alt_name}",
                    }
                }
            }
            div {
                class: "card-body",
                style: if degree.logo.is_some() { "width: 90%;" } else { "width: 100%;" },
                div {
                    class: "body-header",
                    div {
                        class: "header-main-row",
                        h2 {
                            class: "card-title",
                            "{degree.title}"
                        }
                        h3 {
                            class: "duration",
                            "{degree.duration}"
                        }
                    }
                    div {
                        class: "header-subtitle-row",
                        h3 {
                            class: "card-subtitle",
                            "{degree.subtitle}"
                        }
                    }
                }
                div {
                    class: "body-content",
                    for sentence in degree.descriptions.iter() {
                        p {
                            class: "content-list",
                            "{sentence}"
                        }
                    }
                    if let Some(website_link) = degree.website_link {
                        a {
                            href: "{website_link}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            div {
                                class: "visit-btn",
                                class: "btn",
                                "Visit Website"
                            }
                        }
                    }
                }
            }
        }
    }
}
