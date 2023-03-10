use crate::host_header::HostHeader;
use crate::ressources::Version::Version;
use rocket::serde::json::Json;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index(host: HostHeader) -> Template {
    Template::render(
        "index",
        context! {
            name: host.base_domain
        },
    )
}

#[get("/apple")]
pub fn apple(host: HostHeader) -> Template {
    Template::render(
        "apple",
        context! {
            name: host.base_domain
        },
    )
}

#[get("/robots.txt")]
pub fn robots() -> &'static str {
    "User-agent: *\nDisallow: /\n"
}

#[get("/version")]
pub fn version() -> Json<Version<'static>> {
    Json(Version {
        code: env!("VERGEN_BUILD_SEMVER"),
        description: "Wdes Mail AutoDiscover-AutoConfig (https://github.com/wdes/mail-autodiscover-autoconfig)".to_string(),
    })
}

#[get("/ping")]
pub fn ping() -> &'static str {
    "pong"
}
