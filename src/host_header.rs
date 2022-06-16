// Source: https://github.com/SergioBenitez/Rocket/issues/823#issuecomment-1157761369
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::Request;

pub struct HostHeader<'a>(pub &'a str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HostHeader<'r> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Host") {
            Some(h) => Outcome::Success(HostHeader(h)),
            None => Outcome::Forward(()),
        }
    }
}
