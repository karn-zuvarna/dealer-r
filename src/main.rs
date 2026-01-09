use actix_web::{App, HttpServer, web};

mod app;
mod setting;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = setting::Configs::init().expect("ไม่สามารถโหลดการตั้งค่าได้");
    println!("starting on env: {}, port: {}", config.env, config.port);
    HttpServer::new(|| App::new().route("/order", web::post().to(crate::app::order::create_order)))
        .bind(format!("127.0.0.1:{}", config.port))?
        .run()
        .await
}
