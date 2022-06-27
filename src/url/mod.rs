mod service;
use service as url_service;

mod dto;
use dto::{CreateUrlDto, GetUrlReqDto};

use actix_web::{get, post, web, HttpResponse};

use url_shortener_rust::AppState;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/urls")
            .service(self::create_short_url)
            .service(self::get_short_url)
            .service(redirect),
    );
}

#[get("/{url}")]
async fn redirect(state: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let short_url = path.into_inner();

    let full_url = url_service::get_full_url_from_short_url(state, short_url).await;

    HttpResponse::PermanentRedirect()
        .append_header(("LOCATION", full_url))
        .body(())
}

#[post("/")]
async fn create_short_url(state: web::Data<AppState>, req_dto: web::Json<CreateUrlDto>) -> String {
    let req_dto: CreateUrlDto = req_dto.into_inner();

    url_service::create_short_url(state, req_dto).await;

    "Success".to_owned()
}

#[get("/")]
async fn get_short_url(state: web::Data<AppState>) -> HttpResponse {
    let response: GetUrlReqDto = url_service::list_all_short_urls(state).await;

    HttpResponse::Ok().json(response)
}
