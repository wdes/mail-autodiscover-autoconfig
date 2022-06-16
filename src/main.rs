#[macro_use]
extern crate rocket;
extern crate dotenv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use crate::dotenv::dotenv;
use std::env;


#[launch]
fn rocket() -> _ {
    println!("Wdes Mail AutoDiscover-AutoConfig (https://github.com/wdes/mail-autodiscover-autoconfig).");
    dotenv().ok();

    rocket::build()
        .mount(
            "/",
            vec![
            ],
        )
}
