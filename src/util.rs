use std::env;

#[derive(Eq, PartialEq, Debug)]
pub struct Config<'a> {
    pub domain: &'a str,
    pub display_name: String,
    pub imap_hostname: String,
    pub pop_hostname: String,
    pub smtp_hostname: String,
    pub pop_leave_on_server: Option<u32>,
}

pub fn get_custom_domains_list() -> String {
    match env::var("CUSTOM_DOMAINS") {
        Ok(custom_domains) => custom_domains,
        _ => "".to_string(),
    }
}

fn get_pop_leave_on_server_from_env() -> Option<u32> {
    env::var("POP_LEAVE_ON_SERVER").ok().and_then(|val| val.parse::<u32>().ok())
}

pub fn get_config_for_domain(domain: &str) -> Config {
    let is_custom_host: bool = get_custom_domains_list()
        .split(',')
        .collect::<Vec<&str>>()
        .contains(&domain);

    let pop_leave_on_server = get_pop_leave_on_server_from_env();

    if is_custom_host {
        return Config {
            domain: domain,
            display_name: domain.to_owned() + " Mail",
            imap_hostname: "imap.".to_owned() + &domain.to_owned(),
            pop_hostname: "pop.".to_owned() + &domain.to_owned(),
            smtp_hostname: "smtp.".to_owned() + &domain.to_owned(),
            pop_leave_on_server,
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
        pop_leave_on_server: pop_leave_on_server,
    }
}
