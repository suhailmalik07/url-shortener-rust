use actix_web::web;
use entity::url;
use rand::Rng;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use url_shortener_rust::AppState;

use super::dto::{CreateUrlDto, GetUrlReqDto};

fn gen_short_key() -> String {
    let short_key_possible_chars: Vec<_> =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
            .chars()
            .collect();
    let short_key_length: u8 = 7;

    let chars_last_index = short_key_possible_chars.len() - 1;
    let mut short_key = "".to_owned();

    for _ in 0..short_key_length {
        let random_char_index = rand::thread_rng().gen_range(0..=chars_last_index);
        let random_char = short_key_possible_chars[random_char_index];
        short_key.push(random_char);
    }

    short_key
}

pub async fn create_short_url(state: web::Data<AppState>, dto: CreateUrlDto) {
    let short_key = gen_short_key();

    url::ActiveModel {
        short_key: Set(short_key),
        url: Set(dto.url),
        ..Default::default()
    }
    .save(&state.conn)
    .await
    .expect("Couldn't add it");
}

pub async fn list_all_short_urls(state: web::Data<AppState>) -> GetUrlReqDto {
    let urls = url::Entity::find()
        .all(&state.conn)
        .await
        .expect("Something went wrong");

    GetUrlReqDto { list: urls }
}

pub async fn get_full_url_from_short_key(state: web::Data<AppState>, short_url: String) -> String {
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
