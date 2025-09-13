use crate::data::SocialMediaLink;
use dioxus::prelude::*;
const SOCIAL_MEDIA_CSS: Asset = asset!("/src/components/social_media/social_media.css");
#[component]
pub fn SocialMedia(links: Vec<SocialMediaLink>) -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: SOCIAL_MEDIA_CSS,
        }
        div {
            class: "social-media-div",
            for link in links.iter() {
                a {
                    href: "{link.link}",
                    class: "icon-button {link.name.to_lowercase().as_str()}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    i {
                        class: "fa {link.font_awesome_icon}",
                        style: "background-color: {link.background_color};",
                    }
                }
            }
        }
    }
}
