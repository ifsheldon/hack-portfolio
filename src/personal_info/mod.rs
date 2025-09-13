use crate::data::*;
use dioxus::prelude::*;
use std::sync::LazyLock;
const PLACE_HOLDER_IMG_PERSON: Asset = asset!(
    "/src/personal_info/images/confidential.jpg"
);
const PLACE_HOLDER_GENERIC: Asset = asset!("/src/personal_info/images/cg_img.png");
const PLACE_HOLDER_EDU: Asset = asset!(
    "/src/personal_info/images/deeplearning_ai_logo.png"
);
pub const GREETING: LazyLock<Greeting> = LazyLock::new(|| Greeting {
    name: "Ifsheldon",
    logo_name: "Ifsheldon",
    nickname: Some("Shelly"),
    hello_image: PLACE_HOLDER_IMG_PERSON,
    sub_title: "A not-wanna-be bug sweeper",
});
pub const SOCIAL_MEDIA_LINKS: LazyLock<Vec<SocialMediaLink>> = LazyLock::new(|| {
    vec![
        SocialMediaLink {
            name: "Github",
            link: "https://github.com/ifsheldon",
            font_awesome_icon: "fa-brands fa-github",
            background_color: "#181717",
        },
        SocialMediaLink {
            name: "LinkedIn",
            link: "https://www.linkedin.com/",
            font_awesome_icon: "fa-brands fa-linkedin",
            background_color: "#0077B5",
        },
        SocialMediaLink {
            name: "Gmail",
            link: "mailto:hi@reify.ing",
            font_awesome_icon: "fa-solid fa-envelope",
            background_color: "#D14836",
        },
    ]
});
pub const SKILLS: LazyLock<Skills> = LazyLock::new(|| Skills {
    data: vec![
        SkillCategory {
            title: "Computer Vision",
            icon: asset!("/src/personal_info/images/cv.svg"),
            skills: vec![
                "⚡ Working on the instersetion of Computer Vision and Computer Graphics",
                "⚡ Integrating neural networks into computer graphics systems",
                "⚡ Experience of managing CV projects",
            ],
            software_skills: vec![
                SoftwareSkill {
                    skill_name: "PyTorch",
                    iconify_classname: "logos-pytorch",
                },
                SoftwareSkill {
                    skill_name: "Python",
                    iconify_classname: "ion-logo-python",
                },
            ],
        },
        SkillCategory {
            title: "Computer Graphics and Scientific Visualization",
            icon: PLACE_HOLDER_GENERIC,
            skills: vec![
                "⚡ Working on neural rendering",
                "⚡ Working on WebGPU-based renderers and visualization",
                "⚡ Working on applications of differentiable rendering",
            ],
            software_skills: vec![
                SoftwareSkill {
                    skill_name: "Rust",
                    iconify_classname: "logos:rust",
                },
                SoftwareSkill {
                    skill_name: "OpenGL",
                    iconify_classname: "logos-opengl",
                },
                SoftwareSkill {
                    skill_name: "CUDA",
                    iconify_classname: "vscode-icons:file-type-cuda",
                },
            ],
        },
    ],
});
pub const DEGREES: LazyLock<Degrees> = LazyLock::new(|| Degrees {
    degrees: vec![
        Degree {
            title: "Southern University of Science and Technology, China",
            subtitle: "BS in Computer Science and Technology",
            logo: Some(PLACE_HOLDER_EDU),
            alt_name: "SUSTech",
            duration: "2000 - 2019",
            descriptions: vec![
                "⚡ I have studied basic software engineering subjects like DS, Algorithms, DBMS, OS, AI etc.",
                "⚡ Apart from this, I have done courses on Deep Learning, Data Science, Distributed Systems and Computer Graphics.",
            ],
            website_link: Some("https://www.sustech.edu.cn"),
        },
        Degree {
            title: "National University of Singapore",
            subtitle: "CS Summer Workshop - Real-time Rendering",
            logo: Some(PLACE_HOLDER_EDU),
            alt_name: "NUS",
            duration: "2000 - 2019",
            descriptions: vec![
                "⚡ I have studied basics of photorealistic rendering of Computer Graphics like OpenGL and raytracing.",
            ],
            website_link: Some("https://nus.edu.sg"),
        },
    ],
});
pub const CERTIFICATIONS: LazyLock<Certifications> = LazyLock::new(|| Certifications {
    certifications: vec![
        Certification {
            title: "Deep Learning",
            subtitle: "- Andrew Ng",
            logo: PLACE_HOLDER_EDU,
            certificate_link: "https://deeplearning.ai",
            alt_name: "deeplearning.ai",
            color_code: "#00000099",
        },
    ],
});
pub const EXPERIENCE: LazyLock<Experience> = LazyLock::new(|| Experience {
    title: "Career",
    subtitle: "Work and Internship",
    description: "Hello! This is my career experience.",
    sections: vec![
        ExperienceSection {
            title: "Work",
            experiences: vec![
                ExperienceItem {
                    title: "Tech Lead",
                    company: "Example",
                    company_url: "https://www.example.com/",
                    logo: PLACE_HOLDER_EDU,
                    duration: "Feb 2000 - Feb 2019",
                    location: "Earth",
                    description: "Example",
                },
            ],
        },
        ExperienceSection {
            title: "Internships",
            experiences: vec![
                ExperienceItem {
                    title: "Tech Intern",
                    company: "Example",
                    company_url: "https://www.example.com/",
                    logo: PLACE_HOLDER_EDU,
                    duration: "Jul 2000 - Aug 2019",
                    location: "Earth",
                    description: "Example",
                },
            ],
        },
    ],
});
pub const PUBLICATIONS: LazyLock<Publications> = LazyLock::new(|| Publications {
    description: "Your awesome research",
    sections: vec![
        PublicationSection {
            category: "Publications",
            publications: vec![
                PublicationItem {
                    title: "Publication 1",
                    url: None,
                    cover_image: asset!("/src/personal_info/images/cv.svg"),
                    publish_date: None,
                    conference: "Top Conference".into(),
                    r#abstract: "Publication 1".into(),
                    authors: vec!["Author 1", "Author 2", "Author 3"],
                },
            ],
        },
    ],
});
pub const PROJECTS_HEADER: ProjectsHeader = ProjectsHeader {
    title: "Projects",
    description: "I have got many TODOs and WIPs, many of which are interesting! Looking forward to collaborating with YOU!",
};
pub const PROJECTS: LazyLock<Projects> = LazyLock::new(|| Projects {
    data: vec![
        Project {
            name: "stannum",
            url: "https://github.com/ifsheldon/stannum",
            description: "Fusing Taichi into PyTorch",
        },
        Project {
            name: "transprompt",
            url: "https://github.com/ifsheldon/transprompt",
            description: "Prompt-centric framework for developing LLM applications in Rust",
        },
        Project {
            name: "MPL_Lightning",
            url: "https://github.com/ifsheldon/MPL_Lightning",
            description: "Lightning implementation of Meta Pseudo Label",
        },
        Project {
            name: "Differender",
            url: "https://github.com/nanovis/Differender",
            description: "Taichi-based Differentiable SciVis Renderer for PyTorch",
        },
        Project {
            name: "RustyOOPatterns",
            url: "https://github.com/ifsheldon/RustyOOPatterns",
            description: "OO pattern implementations in Rust",
        },
        Project {
            name: "Wenderer",
            url: "https://github.com/nanovis/Wenderer",
            description: "WebGPU-based DVR Renderer",
        },
    ],
});
pub const CONTACT_PAGE_DATA: ContactPageData = ContactPageData {
    contact_section: ContactSection {
        profile_image: PLACE_HOLDER_IMG_PERSON,
        description: "Feel free to contact me. Looking forward to seeing your brilliant ideas on AR/MR, CG and CV!",
    },
    blog_section: BlogSection {
        title: "Blogs",
        subtitle: "Logs for my ideas and nonsense",
        link: "https://github.com/ifsheldon",
    },
    address_section: AddressSection {
        title: "Address",
        subtitle: "Thuwal, Saudi Arabia",
        location_map_link: "https://goo.gl/maps/",
    },
};
