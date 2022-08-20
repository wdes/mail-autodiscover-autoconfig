use rocket::data::ByteUnit;
use rocket::data::{self, Capped, Data, FromData};
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket_dyn_templates::Template;
use serde_derive::Deserialize;
use serde_xml_rs::from_str;
use std::io;

#[derive(Debug)]
pub enum XmlError {
    Io(io::Error),
    Parse(String, serde_xml_rs::Error),
}

#[derive(Deserialize)]
pub struct AutoDiscoverXmlPayloadRequest {
    pub EMailAddress: Option<String>,
    pub LegacyDN: Option<String>,
    pub AcceptableResponseSchema: String,
}

#[derive(Deserialize)]
pub struct AutoDiscoverXmlPayload {
    pub Request: AutoDiscoverXmlPayloadRequest,
}

pub struct AutoDiscoverXmlError {
    pub template: Template,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for AutoDiscoverXmlError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        Response::build_from(self.template.respond_to(req)?)
            .header(ContentType::XML)
            .status(Status::Ok)// Yes, I did curl microsoft and it sends 200 as a response code
            .ok()
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for AutoDiscoverXmlPayload {
    type Error = XmlError;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use rocket::outcome::Outcome::*;
        let size_limit = req.limits().get("xml").unwrap_or(ByteUnit::Kilobyte(4096));

        let contents: Result<Capped<std::string::String>, std::io::Error> =
            data.open(size_limit).into_string().await;
        match contents {
            Ok(dd) => {
                let payload: Result<AutoDiscoverXmlPayload, serde_xml_rs::Error> = from_str(&dd);
                match payload {
                    Ok(d) => Success(d),
                    Err(e) => Failure((
                        Status::UnprocessableEntity,
                        XmlError::Parse(dd.to_string(), e),
                    )),
                }
            }
            Err(e) => Failure((Status::UnprocessableEntity, XmlError::Io(e))),
        }
    }
}

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
