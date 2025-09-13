use crate::components::SocialMedia;
use crate::personal_info::{GREETING, SKILLS, SOCIAL_MEDIA_LINKS};
use dioxus::prelude::*;
const HOME_CSS: Asset = asset!("/src/views/home/home.css");
#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: HOME_CSS,
        }
        div {
            class: "greet-main",
            div {
                class: "greeting-main",
                div {
                    class: "greeting-text-div",
                    h1 {
                        class: "greeting-text",
                        "{GREETING.name}"
                    }
                    if let Some(nickname) = GREETING.nickname {
                        h2 {
                            class: "greeting-nickname",
                            "( {nickname} )"
                        }
                    }
                    p {
                        class: "greeting-text-p subTitle",
                        "{GREETING.sub_title}"
                    }
                    div {
                        class: "social-media-wrapper",
                        SocialMedia {
                            links: SOCIAL_MEDIA_LINKS.clone(),
                        }
                    }
                }
                div {
                    class: "greeting-image-div",
                    img {
                        src: "{GREETING.hello_image}",
                        alt: "{GREETING.name}",
                    }
                }
            }
        }
        div {
            class: "what-i-do-section",
            h1 {
                class: "section-title",
                "What I Do?"
            }
            for (index , skill_category) in SKILLS.data.iter().enumerate() {
                div {
                    class: "skill-section",
                    if index % 2 == 0 {
                        div {
                            class: "skill-section-content",
                            div {
                                class: "skill-section-illustration",
                                img {
                                    src: "{skill_category.icon}",
                                    alt: "{skill_category.title} illustration",
                                }
                            }
                            div {
                                class: "skill-section-info",
                                h2 {
                                    class: "skill-category-title",
                                    "{skill_category.title}"
                                }
                                div {
                                    class: "skill-tech-icons-container",
                                    div {
                                        class: "skill-tech-icons-row",
                                        for software_skill in skill_category.software_skills.iter() {
                                            div {
                                                class: "tech-icon-item",
                                                span {
                                                    class: "iconify",
                                                    "data-icon": "{software_skill.iconify_classname}",
                                                    "data-inline": "false",
                                                }
                                            }
                                        }
                                    }
                                }
                                div {
                                    class: "skill-descriptions",
                                    for skill in skill_category.skills.iter() {
                                        p {

                                            "{skill}"
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        div {
                            class: "skill-section-content reverse",
                            div {
                                class: "skill-section-info",
                                h2 {
                                    class: "skill-category-title",
                                    "{skill_category.title}"
                                }
                                div {
                                    class: "skill-tech-icons-container",
                                    div {
                                        class: "skill-tech-icons-row",
                                        for software_skill in skill_category.software_skills.iter() {
                                            div {
                                                class: "tech-icon-item",
                                                span {
                                                    class: "iconify",
                                                    "data-icon": "{software_skill.iconify_classname}",
                                                    "data-inline": "false",
                                                }
                                            }
                                        }
                                    }
                                }
                                div {
                                    class: "skill-descriptions",
                                    for skill in skill_category.skills.iter() {
                                        p {

                                            "{skill}"
                                        }
                                    }
                                }
                            }
                            div {
                                class: "skill-section-illustration",
                                img {
                                    src: "{skill_category.icon}",
                                    alt: "{skill_category.title} illustration",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
