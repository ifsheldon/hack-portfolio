use dioxus::prelude::*;
// TODO: add SEO
#[derive(Debug, Clone)]
pub struct Seo {
    pub title: &'static str,
    pub description: &'static str,
    pub og: OpenGraph,
}
#[derive(Debug, Clone, PartialEq)]
pub struct OpenGraph {
    pub title: &'static str,
    pub og_type: &'static str,
    pub url: &'static str,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Greeting {
    pub name: &'static str,
    pub logo_name: &'static str,
    pub nickname: Option<&'static str>,
    pub hello_image: Asset,
    pub sub_title: &'static str,
}
#[derive(Debug, Clone, PartialEq)]
pub struct SocialMediaLink {
    pub name: &'static str,
    pub link: &'static str,
    pub font_awesome_icon: &'static str,
    pub background_color: &'static str,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Skills {
    pub data: Vec<SkillCategory>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct SkillCategory {
    pub title: &'static str,
    pub icon: Asset,
    pub skills: Vec<&'static str>,
    pub software_skills: Vec<SoftwareSkill>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct SoftwareSkill {
    pub skill_name: &'static str,
    pub iconify_classname: &'static str,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Degrees {
    pub degrees: Vec<Degree>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Degree {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub logo: Option<Asset>,
    pub alt_name: &'static str,
    pub duration: &'static str,
    pub descriptions: Vec<&'static str>,
    pub website_link: Option<&'static str>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Certifications {
    pub certifications: Vec<Certification>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Certification {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub logo: Asset,
    pub certificate_link: &'static str,
    pub alt_name: &'static str,
    pub color_code: &'static str,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Experience {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub description: &'static str,
    pub sections: Vec<ExperienceSection>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ExperienceSection {
    pub title: &'static str,
    pub experiences: Vec<ExperienceItem>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ExperienceItem {
    pub title: &'static str,
    pub company: &'static str,
    pub company_url: &'static str,
    pub logo: Asset,
    pub duration: &'static str,
    pub location: &'static str,
    pub description: &'static str,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectsHeader {
    pub title: &'static str,
    pub description: &'static str,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Projects {
    pub data: Vec<Project>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub name: &'static str,
    pub url: &'static str,
    pub description: &'static str,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ContactPageData {
    pub contact_section: ContactSection,
    pub blog_section: BlogSection,
    pub address_section: AddressSection,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ContactSection {
    pub profile_image: Asset,
    pub description: &'static str,
}
#[derive(Debug, Clone, PartialEq)]
pub struct BlogSection {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub link: &'static str,
}
#[derive(Debug, Clone, PartialEq)]
pub struct AddressSection {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub location_map_link: &'static str,
}
