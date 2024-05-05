use actix_web::{App, HttpServer};
use tracing_actix_web::TracingLogger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    HttpServer::new(|| App::new().wrap(TracingLogger::default()).service(root))
        .bind(("0.0.0.0", 3000))?
        .run()
        .await?;
    Ok(())
}

#[actix_web::get("/")]
async fn root() -> &'static str {
    "hello world"
}
