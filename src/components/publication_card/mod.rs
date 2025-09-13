use crate::data::PublicationItem;
use dioxus::prelude::*;
const PUBLICATION_CARD_CSS: Asset = asset!(
    "/src/components/publication_card/publication_card.css"
);
#[component]
pub fn PublicationCard(publication: PublicationItem) -> Element {
    let mut is_open = use_signal(|| false);
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: PUBLICATION_CARD_CSS,
        }
        div {
            class: "publication-card",
            div {
                class: "publication-card-cover-div",
                img {
                    src: "{publication.cover_image}",
                    class: "publication-card-cover",
                    alt: "{publication.title}",
                }
            }
            div {
                class: "publication-card-body-div",
                div {
                    class: "publication-card-header-div",
                    div {
                        class: "publication-card-heading-left",
                        h3 {
                            class: "publication-card-title",
                            if let Some(url) = publication.url {
                                a {
                                    href: "{url}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    "{publication.title}"
                                }
                            } else {
                                "{publication.title}"
                            }
                        }
                        if !publication.authors.is_empty() {
                            p {
                                class: "publication-card-authors",
                                "{publication.authors.join(\", \")}"
                            }
                        }
                    }
                    div {
                        class: "publication-card-heading-right",
                        if let Some(conf) = publication.conference {
                            p {
                                class: "publication-card-venue",
                                "{conf}"
                            }
                        }
                        if let Some(date) = publication.publish_date {
                            p {
                                class: "publication-card-date",
                                "{date}"
                            }
                        }
                    }
                }
                if publication.r#abstract.is_some() {
                    button {
                        class: if is_open() { "publication-card-toggle open" } else { "publication-card-toggle" },
                        r#type: "button",
                        aria_expanded: "{is_open()}",
                        onclick: move |_| {
                            is_open.set(!is_open());
                        },
                        i {
                            class: "fa-solid fa-chevron-down",
                        }
                    }
                }
            }
            if let Some(abs) = publication.r#abstract {
                div {
                    class: if is_open() { "publication-card-abstract-wrapper open" } else { "publication-card-abstract-wrapper" },
                    div {
                        class: "publication-card-abstract",
                        h4 {
                            class: "publication-card-abstract-title",
                            "Abstract"
                        }
                        div {
                            class: "publication-card-abstract-content",
                            "{abs}"
                        }
                    }
                }
            }
        }
    }
}
