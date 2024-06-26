use actix_web::{web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use tracing_actix_web::TracingLogger;

mod services;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // get env vars
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    // let host = std::env::var("HOST").expect("HOST is not set in .env file");
    // let port = std::env::var("PORT").expect("PORT is not set in .env file");
    // let server_url = format!("{host}:{port}");

    // establish connection to database and apply migrations
    let connection = Database::connect(&db_url).await?;
    Migrator::up(&connection, None).await?;

    let state = AppState { connection };

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(state.clone()))
            .service(web::resource(["/", "/forum/{id}/"]).get(services::index))
            .service(
                web::resource(["/new-forum", "/forum/{id}/new-forum"])
                    .route(web::get().to(services::new_forum_get))
                    .route(web::post().to(services::new_forum_post)),
            )
            .service(
                web::resource("/forum/{id}/new-thread")
                    .route(web::get().to(services::new_thread_get))
                    .route(web::post().to(services::new_thread_post)),
            )
            .service(web::resource("/thread/{id}/").get(services::view_thread))
            .service(
                web::resource("/thread/{id}/reply")
                    .route(web::get().to(services::reply_get))
                    .route(web::post().to(services::reply_post)),
            )
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await?;
    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    connection: DatabaseConnection,
}
