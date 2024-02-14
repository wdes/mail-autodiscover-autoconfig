use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket_dyn_templates::Template;

pub struct AppleResponse {
    pub template: Template,
    pub domain: String,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for AppleResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        Response::build_from(self.template.respond_to(req)?)
            .raw_header(
                "Content-Type",
                "application/x-apple-aspen-config; charset=utf-8",
            )
            .raw_header(
                "Content-Disposition",
                "attachment; filename=\"".to_owned() + &self.domain + ".mobileconfig\"",
            )
            .ok()
    }
}
