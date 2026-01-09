use actix_web::{App, HttpServer, web};

mod setting;
mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match setting::Configs::init() {
        Ok(config) => {
            println!("Config: {:?}", config);
            println!("Running on port: {}", config.port);
            println!("Environment: {}", config.env);
        }
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    }

    HttpServer::new(|| App::new().route("/order", web::post().to(crate::app::order::create_order)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
