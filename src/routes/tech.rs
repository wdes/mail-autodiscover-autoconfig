use crate::host_header::HostHeader;
use crate::ressources::Version::Version;
use rocket::serde::json::Json;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index(host: HostHeader) -> Template {
    Template::render(
        "index",
        context! {
            name: host.0
        },
    )
}

#[get("/robots.txt")]
pub fn robots() -> Template {
    Template::render("static/robots", context! {})
}

#[get("/version")]
pub fn version() -> Json<Version> {
    Json(Version {
        code: "v1".to_string(),
        description: "Wdes Mail AutoDiscover-AutoConfig (https://github.com/wdes/mail-autodiscover-autoconfig)".to_string(),
    })
}
