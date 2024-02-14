use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version<'a> {
    pub code: &'a str,
    pub description: String,
}
