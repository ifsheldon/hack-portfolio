use dioxus::prelude::*;
use crate::components::{DegreeCard, CertificationCard};
use crate::personal_info::{DEGREES, CERTIFICATIONS};
const EDUCATION_CSS: Asset = asset!("/src/views/education/education.css");
#[component]
pub fn Education() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: EDUCATION_CSS,
        }
        div {
            class: "education-container",
            h1 {

                "Education"
            }
            div {
                class: "degrees-container",
                for degree in DEGREES.degrees.iter() {
                    DegreeCard {
                        degree: degree.clone(),
                    }
                }
            }
            h1 {

                "Certifications"
            }
            div {
                class: "certifications-container",
                for certification in CERTIFICATIONS.certifications.iter() {
                    CertificationCard {
                        certification: certification.clone(),
                    }
                }
            }
        }
    }
}
