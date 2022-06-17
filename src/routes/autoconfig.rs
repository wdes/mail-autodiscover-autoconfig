use crate::host_header::HostHeader;
use crate::ressources::AutoDiscoverJson::AutoDiscoverJson;
use rocket::serde::json::Json;
use rocket_dyn_templates::{context, Template};
use std::env;

struct Config<'a> {
    domain: &'a str,
    display_name: String,
    imap_hostname: String,
    pop_hostname: String,
    smtp_hostname: String,
}

fn get_config_for_domain(domain: &str) -> Config {
    let is_custom_host: bool = env::var("CUSTOM_DOMAINS")
        .expect("CUSTOM_DOMAINS must be set")
        .split(',')
        .collect::<Vec<&str>>()
        .contains(
            &domain
                .replace("autoconfig.", "")
                .replace("autodiscover.", "")
                .as_str(),
        );

    if is_custom_host {
        return Config {
            domain: domain,
            display_name: domain.to_owned() + " Mail",
            imap_hostname: "imap.".to_owned() + &domain.to_owned(),
            pop_hostname: "pop.".to_owned() + &domain.to_owned(),
            smtp_hostname: "smtp.".to_owned() + &domain.to_owned(),
        };
    }

    let imap_hostname: String = env::var("IMAP_HOSTNAME").expect("IMAP_HOSTNAME must be set");
    let pop_hostname: String = env::var("POP_HOSTNAME").expect("IMAP_HOSTNAME must be set");
    let smtp_hostname: String = env::var("SMTP_HOSTNAME").expect("IMAP_HOSTNAME must be set");

    Config {
        domain: domain,
        display_name: domain.to_owned() + " Mail",
        imap_hostname: imap_hostname,
        pop_hostname: pop_hostname,
        smtp_hostname: smtp_hostname,
    }
}

fn handle_mail_config_v11(host: HostHeader) -> Template {
    let config: Config = get_config_for_domain(host.0);
    Template::render(
        "xml/config-v1.1",
        context! {
            domain: config.domain,
            display_name: config.display_name,
            imap_hostname: config.imap_hostname,
            pop_hostname: config.pop_hostname,
            smtp_hostname: config.smtp_hostname,
        },
    )
}

// Used by Thunderbird (tested with: Thunderbird 91.10.0)
#[get("/mail/config-v1.1.xml?<emailaddress>")]
#[allow(unused_variables)]
pub fn mail_config_v11(host: HostHeader, emailaddress: Option<&str>) -> Template {
    handle_mail_config_v11(host)
}

#[get("/.well-known/autoconfig/mail/config-v1.1.xml?<emailaddress>")]
#[allow(unused_variables)]
pub fn well_known_mail_config_v11(host: HostHeader, emailaddress: Option<&str>) -> Template {
    handle_mail_config_v11(host)
}

// Used by Microsoft Outlook for Android (tested version: 4.2220.1)
// Example: /autodiscover/autodiscover.json?Email=test%40wdes.fr&Protocol=ActiveSync&RedirectCount=1
#[get("/autodiscover/autodiscover.json?<Email>&<Protocol>&<RedirectCount>")]
#[allow(unused_variables)]
#[allow(non_snake_case)]
pub fn post_mail_autodiscover_microsoft_json(
    host: HostHeader,
    Email: Option<&str>,
    Protocol: Option<&str>,
    RedirectCount: Option<&str>,
) -> Json<Option<AutoDiscoverJson>> {
    // TODO: else respond this
    // {"ErrorCode":"InvalidProtocol","ErrorMessage":"The given protocol value \u0027' . preg_replace("/[^\da-z]/i", '', $_GET['Protocol']) . '\u0027 is invalid. Supported values are \u0027ActiveSync,AutodiscoverV1\u0027"}
    Json(match Protocol {
        None => None,
        Some("AutodiscoverV1") => Some(AutoDiscoverJson {
            Protocol: "AutodiscoverV1".to_string(),
            Url: "https://".to_owned() + host.0 + "/Autodiscover/Autodiscover.xml",
        }),
        Some("ActiveSync") => Some(AutoDiscoverJson {
            Protocol: "ActiveSync".to_string(),
            Url: "https://".to_owned() + host.0 + "/Microsoft-Server-ActiveSync",
        }),
        Some(&_) => None,
    })
}

fn autodiscover_microsoft(host: HostHeader) -> Template {
    let config: Config = get_config_for_domain(host.0);
    Template::render(
        "xml/autodiscover",
        context! {
            domain: config.domain,
            display_name: config.display_name,
            imap_hostname: config.imap_hostname,
            pop_hostname: config.pop_hostname,
            smtp_hostname: config.smtp_hostname,
        },
    )
}

#[get("/autodiscover/autodiscover.xml")]
pub fn mail_autodiscover_microsoft(host: HostHeader) -> Template {
    autodiscover_microsoft(host)
}

#[get("/Autodiscover/Autodiscover.xml")]
pub fn mail_autodiscover_microsoft_case(host: HostHeader) -> Template {
    autodiscover_microsoft(host)
}

#[get("/AutoDiscover/AutoDiscover.xml")]
pub fn mail_autodiscover_microsoft_camel_case(host: HostHeader) -> Template {
    autodiscover_microsoft(host)
}

// Used by Thunderbird (tested version: 91.10.0)
// Used by Microsoft Outlook for Android (tested version: 4.2220.1)
#[post("/autodiscover/autodiscover.xml")]
pub fn post_mail_autodiscover_microsoft(host: HostHeader) -> Template {
    autodiscover_microsoft(host)
}

#[post("/Autodiscover/Autodiscover.xml")]
pub fn post_mail_autodiscover_microsoft_case(host: HostHeader) -> Template {
    autodiscover_microsoft(host)
}

#[post("/AutoDiscover/AutoDiscover.xml")]
pub fn post_mail_autodiscover_microsoft_camel_case(host: HostHeader) -> Template {
    autodiscover_microsoft(host)
}

// iOS / Apple Mail (/email.mobileconfig?email=username@domain.com or /email.mobileconfig?email=username)
#[get("/email.mobileconfig?<email>")]
pub fn mail_autodiscover_microsoft_apple(host: HostHeader, email: &str) -> Template {
    let config: Config = get_config_for_domain(host.0);
    let mail_uuid: String = env::var("APPLE_MAIL_UUID").expect("APPLE_MAIL_UUID must be set");
    let profile_uuid: String =
        env::var("APPLE_PROFILE_UUID").expect("APPLE_PROFILE_UUID must be set");

    // set("Content-Type", "application/x-apple-aspen-config; charset=utf-8");
    // set("Content-Disposition", `attachment; filename="${domain}.mobileconfig"`);

    // See :https://developer.apple.com/business/documentation/Configuration-Profile-Reference.pdf
    Template::render(
        "xml/email_mobileconfig",
        context! {
            domain: config.domain,
            display_name: config.display_name,
            imap_hostname: config.imap_hostname,
            pop_hostname: config.pop_hostname,
            smtp_hostname: config.smtp_hostname,
            email_address: email,
            username: email,
            mail_uuid: mail_uuid,
            profile_uuid: profile_uuid,
        },
    )
}
