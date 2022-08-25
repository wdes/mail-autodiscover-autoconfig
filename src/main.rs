#[macro_use]
extern crate rocket;
extern crate dotenv;
extern crate rocket_dyn_templates;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tera;

use crate::dotenv::dotenv;
use rocket_dyn_templates::Template;
use std::env;

pub mod host_header;
pub mod ressources;
pub mod routes;

#[launch]
fn rocket() -> _ {
    println!(
        "Wdes Mail AutoDiscover-AutoConfig (https://github.com/wdes/mail-autodiscover-autoconfig)."
    );
    dotenv().ok();

    let custom_domains: String = env::var("CUSTOM_DOMAINS").expect("CUSTOM_DOMAINS must be set");
    println!("Custom domains: {}", custom_domains);

    let figment = rocket::Config::figment().merge(("ident", "Wdes Mail AutoDiscover-AutoConfig"));

    let mut rocket = rocket::custom(figment).attach(Template::fairing()).mount(
        "/",
        routes![
            routes::tech::index,
            routes::tech::robots,
            routes::tech::version,
            routes::autoconfig::v11_mail_config_v11,
            routes::autoconfig::mail_config_v11,
            routes::autoconfig::well_known_mail_config_v11,
            routes::autoconfig::mail_autodiscover_microsoft,
            routes::autoconfig::mail_autodiscover_microsoft_case,
            routes::autoconfig::mail_autodiscover_microsoft_camel_case,
            routes::autoconfig::post_mail_autodiscover_microsoft,
            routes::autoconfig::post_mail_autodiscover_microsoft_case,
            routes::autoconfig::post_mail_autodiscover_microsoft_camel_case,
            routes::autoconfig::post_mail_autodiscover_microsoft_json,
        ],
    );
    if cfg!(feature = "apple") {
        rocket = rocket.mount(
            "/",
            routes![
                routes::autoconfig::mail_autodiscover_apple_mobileconfig,
                routes::tech::apple,
            ],
        );
    }
    rocket
}
