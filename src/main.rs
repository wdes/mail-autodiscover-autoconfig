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

pub mod host_header;
pub mod ressources;
pub mod routes;

#[launch]
fn rocket() -> _ {
    println!(
        "Wdes Mail AutoDiscover-AutoConfig (https://github.com/wdes/mail-autodiscover-autoconfig)."
    );
    dotenv().ok();

    let figment = rocket::Config::figment().merge(("ident", "Wdes Mail AutoDiscover-AutoConfig"));

    rocket::custom(figment).attach(Template::fairing()).mount(
        "/",
        routes![
            routes::tech::index,
            routes::tech::robots,
            routes::tech::version,
            routes::autoconfig::mail_config_v11,
            routes::autoconfig::mail_autodiscover_microsoft,
            routes::autoconfig::mail_autodiscover_microsoft_case,
            routes::autoconfig::mail_autodiscover_microsoft_camel_case,
            routes::autoconfig::post_mail_autodiscover_microsoft,
            routes::autoconfig::post_mail_autodiscover_microsoft_case,
            routes::autoconfig::post_mail_autodiscover_microsoft_camel_case,
            routes::autoconfig::mail_autodiscover_microsoft_apple,
        ],
    )
}
