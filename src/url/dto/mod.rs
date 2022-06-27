use entity::url;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUrlDto {
    pub url: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetUrlReqDto {
    pub list: Vec<url::Model>,
}
