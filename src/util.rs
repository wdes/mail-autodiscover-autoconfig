use std::env;

#[derive(Eq, PartialEq, Debug)]
pub struct Config<'a> {
    pub domain: &'a str,
    pub display_name: String,
    pub imap_hostname: String,
    pub pop_hostname: String,
    pub smtp_hostname: String,
}

pub fn get_config_for_domain(domain: &str) -> Config {
    let is_custom_host: bool = env::var("CUSTOM_DOMAINS")
        .expect("CUSTOM_DOMAINS must be set")
        .split(',')
        .collect::<Vec<&str>>()
        .contains(&domain);

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
