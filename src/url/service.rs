use actix_web::web::{self};
use entity::url;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
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

pub async fn list_all_short_urls(state: web::Data<AppState>) -> GetUrlReqDto {
    let urls = url::Entity::find()
        .all(&state.conn)
        .await
        .expect("Something went wrong");

    GetUrlReqDto { list: urls }
}

pub async fn get_full_url_from_short_url(state: web::Data<AppState>, short_url: String) -> String {
    let urls = url::Entity::find()
        .filter(url::Column::ShortKey.eq(short_url))
        .one(&state.conn)
        .await
        .expect("Something went wrong");

    match urls {
        Some(url) => url.url,
        _ => "Figma".to_owned(),
    }
}
