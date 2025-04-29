use dioxus::prelude::*;
use crate::views::{Career, Contact, Education, Home, Navbar, Projects};
mod components;
mod data;
mod views;
mod personal_info;
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/home")]
    HomeRedirect {},
    #[route("/career")]
    Career {},
    #[route("/education")]
    Education {},
    #[route("/contact")]
    Contact {},
    #[route("/projects")]
    Projects {},
    #[route("/*")]
    NotFound {},
}
const GLOBAL_CSS: Asset = asset!("/src/global.css");
const MAIN_CSS: Asset = asset!("/src/main.css");
const FONT_AWESOME_CSS_PATH: &str = "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.7.2/css/all.min.css";
const ICONIFY_JS_PATH: &str = "https://code.iconify.design/1/1.0.7/iconify.min.js";
fn main() {
    launch(App);
}
#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet {
            href: GLOBAL_CSS,
        }
        document::Stylesheet {
            href: MAIN_CSS,
        }
        document::Stylesheet {
            href: FONT_AWESOME_CSS_PATH,
        }
        document::Script {
            src: ICONIFY_JS_PATH,
        }
        Router::<Route> {

        }
    }
}
#[component]
fn HomeRedirect() -> Element {
    use_navigator().push(Route::Home {});
    rsx! {
        div {

            "Redirecting..."
        }
    }
}
#[component]
fn NotFound() -> Element {
    rsx! {
        div {
            class: "not-found",
            h1 {

                "404"
            }
            h2 {

                "Page Not Found"
            }
            p {

                "The page you are looking for doesn't exist or has been moved."
            }
            a {
                href: "/",
                "Go Home"
            }
        }
    }
}
