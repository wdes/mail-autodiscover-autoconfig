use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AutoDiscoverJson {
    pub Protocol: String,
    pub Url: String,
}

#[derive(Serialize, Deserialize)]
pub struct AutoDiscoverJsonError {
    pub ErrorCode: String,
    pub ErrorMessage: String,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for AutoDiscoverJsonError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        Response::build_from(
            Json(AutoDiscoverJsonError {
                ErrorCode: self.ErrorCode,
                ErrorMessage: self.ErrorMessage,
            })
            .respond_to(req)?,
        )
        .header(ContentType::JSON)
        .status(Status::BadRequest)
        .ok()
    }
}
