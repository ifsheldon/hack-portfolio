use crate::views::{Career, Contact, Education, Home, Navbar, Projects, Publications};
use dioxus::prelude::*;
mod components;
mod data;
mod personal_info;
mod views;
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
    #[route("/publications")]
    Publications {},
    #[route("/*")]
    NotFound {},
}
const GLOBAL_CSS: Asset = asset!("/src/global.css");
const MAIN_CSS: Asset = asset!("/src/main.css");
const FONT_AWESOME_CSS_PATH: &str =
    "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.7.2/css/all.min.css";
const ICONIFY_JS_PATH: &str = "https://code.iconify.design/1/1.0.7/iconify.min.js";

#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect())
}

fn main() {
    dioxus::LaunchBuilder::new()
        // Set the server config only if we are building the server target
        .with_cfg(server_only! {
            ServeConfig::builder()
                // Enable incremental rendering
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        // Store static files in the public directory where other static assets like wasm are stored
                        .static_dir(
                            std::env::current_exe()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .join("public")
                        )
                        // Don't clear the public folder on every build. The public folder has other files including the wasm
                        // binary and static assets required for the app to run
                        .clear_cache(false)
                )
                .enable_out_of_order_streaming()
        })
        .launch(App);
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
