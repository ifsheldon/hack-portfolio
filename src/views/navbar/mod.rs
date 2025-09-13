use crate::Route;
use dioxus::prelude::*;
const NAVBAR_CSS: Asset = asset!("/src/views/navbar/navbar.css");
use crate::personal_info::GREETING;
#[component]
pub fn Navbar() -> Element {
    let route = use_route::<Route>();
    let is_active = |r: Route| -> bool {
        match (&route, &r) {
            (Route::Home {}, Route::Home {}) => true,
            (Route::Education {}, Route::Education {}) => true,
            (Route::Career {}, Route::Career {}) => true,
            (Route::Projects {}, Route::Projects {}) => true,
            (Route::Contact {}, Route::Contact {}) => true,
            (Route::Publications {}, Route::Publications {}) => true,
            _ => false,
        }
    };
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: NAVBAR_CSS,
        }
        header {
            class: "header",
            div {
                class: "header-content",
                div {
                    class: "logo",
                    Link {
                        to: Route::Home {},
                        span {

                            "<"
                        }
                        span {

                            "{GREETING.logo_name}"
                        }
                        span {

                            "/>"
                        }
                    }
                }
                nav {
                    class: "navigation",
                    Link {
                        class: if is_active(Route::Home {}) { "active" } else { "" },
                        to: Route::Home {},
                        "Home"
                    }
                    Link {
                        class: if is_active(Route::Education {}) { "active" } else { "" },
                        to: Route::Education {},
                        "Education"
                    }
                    Link {
                        class: if is_active(Route::Career {}) { "active" } else { "" },
                        to: Route::Career {},
                        "Career"
                    }
                    Link {
                        class: if is_active(Route::Projects {}) { "active" } else { "" },
                        to: Route::Projects {},
                        "Projects"
                    }
                    Link {
                        class: if is_active(Route::Publications {}) { "active" } else { "" },
                        to: Route::Publications {},
                        "Publications"
                    }
                    Link {
                        class: if is_active(Route::Contact {}) { "active" } else { "" },
                        to: Route::Contact {},
                        "Contact"
                    }
                }
            }
        }
        Outlet::<Route> {

        }
    }
}
