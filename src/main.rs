use actix_web::{self, middleware::Logger, web, App, HttpServer};
use dotenv;
use sea_orm::{ConnectOptions, Database};
use std::{env, time::Duration};
use url_shortener_rust::AppState;

mod url;
use url as url_module;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap();

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(10)
        .min_connections(0)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    let conn = Database::connect(opt).await.unwrap();

    let state = AppState { conn };

    const PORT: u16 = 8080;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("Request Pre"))
            .app_data(web::Data::new(state.clone()))
            .configure(url_module::configure)
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
