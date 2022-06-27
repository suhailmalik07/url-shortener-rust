use actix_web::web::{self};
use entity::url;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use url_shortener_rust::AppState;

use super::dto::{CreateUrlDto, GetUrlReqDto};

pub async fn create_short_url(state: web::Data<AppState>, dto: CreateUrlDto) {
    url::ActiveModel {
        short_key: Set("abc".to_owned()),
        url: Set(dto.url),
        ..Default::default()
    }
    .save(&state.conn)
    .await
    .expect("Counldn't add it");
}

pub async fn get_short_url(state: web::Data<AppState>) -> GetUrlReqDto {
    let urls = url::Entity::find()
        .all(&state.conn)
        .await
        .expect("Something went wrong");

    GetUrlReqDto { list: urls }
}
