use crate::host_header::HostHeader;
use crate::resources::DnsTxtResponse::DnsTxtResponse;
use crate::util::{get_config_for_domain, Config};
use rocket_dyn_templates::{context, Template};

#[get("/dns-zone")]
pub fn dns_txt_zone(host: HostHeader) -> DnsTxtResponse {
    let config: Config = get_config_for_domain(&host.base_domain);

    // See :https://developer.apple.com/business/documentation/Configuration-Profile-Reference.pdf
    DnsTxtResponse {
        domain: config.domain.to_string(),
        template: Template::render(
            "dns/zone",
            context! {
                imap_hostname: config.imap_hostname,
                pop_hostname: config.pop_hostname,
                smtp_hostname: config.smtp_hostname,
                pop_leave_on_server: config.pop_leave_on_server,
            },
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
