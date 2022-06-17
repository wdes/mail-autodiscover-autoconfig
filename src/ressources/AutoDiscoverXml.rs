use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket_dyn_templates::Template;

pub struct AutoDiscoverXml {
    pub template: Template,
    pub domain: String,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for AutoDiscoverXml {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        let header_accept: Option<&str> = req.headers().get_one("Accept");
        let mime_type: &str = match header_accept {
            Some("text/xml") => "text/xml",
            _ => "application/xml",
        };
        Response::build_from(self.template.respond_to(req)?)
            .raw_header("Content-Type", mime_type)
            .ok()
    }
}
