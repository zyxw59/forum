use actix_web::{App, HttpServer, web};
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
            .service(services::toplevel_forums)
            .service(services::get_create_forum)
            .service(services::post_create_forum)
            .service(services::get_forum)
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
