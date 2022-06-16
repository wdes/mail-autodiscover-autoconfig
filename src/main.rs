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

pub mod ressources;
pub mod routes;

#[launch]
fn rocket() -> _ {
    println!(
        "Wdes Mail AutoDiscover-AutoConfig (https://github.com/wdes/mail-autodiscover-autoconfig)."
    );
    dotenv().ok();

    rocket::build().attach(Template::fairing()).mount(
        "/",
        routes![
            routes::tech::index,
            routes::tech::robots,
            routes::tech::version,
        ],
    )
}
