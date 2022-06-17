use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AutoDiscoverJson {
    pub Protocol: String,
    pub Url: String,
}
