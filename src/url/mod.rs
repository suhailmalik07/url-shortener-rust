pub mod url_controller {
    use actix_web::{web, Scope};

    use super::url_service;

    pub fn scope() -> Scope {
        web::scope("/urls").service(url_service::get_short_url)
    }
}

pub mod url_service {
    use actix_web::{post, web};
    use entity::url;
    use sea_orm::{ActiveModelTrait, Set};
    use url_shortener_rust::AppState;

    #[post("/")]
    async fn get_short_url(state: web::Data<AppState>) -> String {
        println!("Creating something new.");

        url::ActiveModel {
            short_key: Set("abc".to_owned()),
            url: Set("https://www.google.com".to_owned()),
            ..Default::default()
        }
        .save(&state.conn)
        .await
        .expect("Counldn't add it");

        "figma".to_owned()
    }
}
