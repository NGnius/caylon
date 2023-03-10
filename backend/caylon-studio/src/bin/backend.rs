use actix_web::{web, App, HttpServer};

use simplelog::{ColorChoice, LevelFilter, TermLogger, TerminalMode};

use caylon_studio::api::{hits, index, resource, IndexPage};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    TermLogger::init(
        LevelFilter::Debug,
        Default::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
    log::info!("Starting HTTP server @ 127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(IndexPage::load("dist/index.html").unwrap()))
            .service(index)
            .service(resource)
            .service(hits)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
