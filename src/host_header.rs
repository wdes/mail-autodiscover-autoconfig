// Source: https://github.com/SergioBenitez/Rocket/issues/823#issuecomment-1157761369
use addr::parse_domain_name;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::Request;

pub struct HostHeader<'a> {
    pub host: &'a str,
    pub base_domain: String,
}

fn get_base_domain<'a>(header: &'a str) -> String {
    let parsed = &parse_domain_name(header);

    match parsed {
        Ok(h) => match h.root() {
            Some(d) => d.to_string(),
            None => header.to_string(),
        },
        _ => header.to_string(),
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HostHeader<'r> {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let header_name: &str = match req.headers().contains("X-Forwarded-Host") {
            true => "X-Forwarded-Host",
            false => "Host",
        };

        match req.headers().get_one(header_name) {
            Some(h) => Outcome::Success(HostHeader {
                host: h,
                base_domain: get_base_domain(h),
            }),
            None => Outcome::Error((
                Status::UnprocessableEntity,
                "Missing a Host or X-Forwarded-Host header".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_base_domain() {
        assert_eq!("d", get_base_domain("d"));
        assert_eq!("d.tld", get_base_domain("d.tld"));
        assert_eq!("d.tld", get_base_domain("foo.d.tld"));
        assert_eq!("wdes.fr", get_base_domain("autoconfig.wdes.fr"));
    }
}
