use crate::data::Certification;
use dioxus::prelude::*;
const CERTIFICATION_CARD_CSS: Asset = asset!(
    "/src/components/certification_card/certification_card.css"
);
#[component]
pub fn CertificationCard(certification: Certification) -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: CERTIFICATION_CARD_CSS,
        }
        div {
            class: "cert-card",
            div {
                class: "content",
                a {
                    href: "{certification.certificate_link}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    div {
                        class: "content-overlay",
                        style: "background-color: {certification.color_code};",
                    }
                    div {
                        class: "cert-header",
                        img {
                            class: "logo_img",
                            src: "{certification.logo}",
                            alt: "{certification.alt_name}",
                        }
                    }
                    div {
                        class: "cert-body",
                        h2 {
                            class: "cert-body-title",
                            "{certification.title}"
                        }
                        h3 {
                            class: "cert-body-subtitle",
                            "{certification.subtitle}"
                        }
                    }
                }
            }
        }
    }
}
