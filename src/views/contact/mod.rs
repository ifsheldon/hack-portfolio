use crate::components::SocialMedia;
use crate::personal_info::{CONTACT_PAGE_DATA, GREETING, SOCIAL_MEDIA_LINKS};
use dioxus::prelude::*;
const CONTACT_CSS: Asset = asset!("/src/views/contact/contact.css");
const AVARTAR_IMG: Asset = asset!("/prefabs/address_image.svg");
const BLOG_IMG: Asset = asset!("/prefabs/blogs_image.svg");
#[component]
pub fn Contact() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: CONTACT_CSS,
        }
        div {
            class: "contact-container",
            div {
                class: "contact-section",
                div {
                    class: "contact-content",
                    div {
                        class: "contact-image",
                        img {
                            src: "{CONTACT_PAGE_DATA.contact_section.profile_image}",
                            alt: "{GREETING.name}",
                        }
                    }
                    div {
                        class: "contact-text",
                        p {

                            "{CONTACT_PAGE_DATA.contact_section.description}"
                        }
                        div {
                            class: "social-media-container",
                            SocialMedia {
                                links: SOCIAL_MEDIA_LINKS.clone(),
                            }
                        }
                    }
                }
            }
            div {
                class: "info-sections",
                div {
                    class: "blog-section",
                    h1 {

                        "{CONTACT_PAGE_DATA.blog_section.title}"
                    }
                    p {

                        "{CONTACT_PAGE_DATA.blog_section.subtitle}"
                    }
                    div {
                        class: "blog-image",
                        img {
                            src: "{BLOG_IMG}",
                            alt: "Blog Illustration",
                        }
                    }
                    a {
                        href: "{CONTACT_PAGE_DATA.blog_section.link}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        div {
                            class: "blog-button",
                            "Visit Blog"
                        }
                    }
                }
                div {
                    class: "address-section",
                    h1 {

                        "{CONTACT_PAGE_DATA.address_section.title}"
                    }
                    p {

                        "{CONTACT_PAGE_DATA.address_section.subtitle}"
                    }
                    div {
                        class: "address-image",
                        img {
                            src: "{AVARTAR_IMG}",
                            alt: "Address Illustration",
                        }
                    }
                    a {
                        href: "{CONTACT_PAGE_DATA.address_section.location_map_link}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        div {
                            class: "map-button",
                            "View on Google Maps"
                        }
                    }
                }
            }
        }
    }
}
