pub(crate) mod api;
pub(crate) mod models;

use actix_web::middleware::{Compress, Logger, NormalizePath, TrailingSlash};
use actix_web::{App, HttpServer};
use actix_web::web::{Data, scope};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

fn init_logger() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    tracing_subscriber::fmt().json().init();
}

pub async fn init_pool() -> Pool<Postgres> {
    let app_config = models::app_config::AppConfig::new();
    let pool = PgPoolOptions::new()
        .max_connections(app_config.database_pool_size)
        .connect(&app_config.database_url)
        .await
        .expect(format!("Error building a connection pool to the database {}", app_config.database_url).as_str());
    pool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    let pool = init_pool().await;
    let app_state = models::app_state::AppState { pool };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(app_state.clone()))
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::MergeOnly))
            .service(api::health::healthcheck)
            .service(
                scope("/api/v1")
                    .service(api::users::users)
                    .service(api::users::find_user)
                    .service(api::users::create_user)
            )
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
