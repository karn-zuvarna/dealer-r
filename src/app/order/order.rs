use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct OrderRequest {
    symbol: String,
    quantity: f64,
}

// 2. นิยามข้อมูลที่จะส่งออก (ใช้ Serialize)
#[derive(Serialize)]
struct OrderResponse {
    status: String,
    message: String,
}

// 3. ฟังก์ชัน Handler สำหรับ POST
pub async fn create_order(item: web::Json<OrderRequest>) -> impl Responder {
    // เข้าถึงข้อมูลได้ง่ายๆ ผ่าน .into_inner() หรือ .property
    println!("ได้รับคำสั่งซื้อ: {} จำนวน {}", item.symbol, item.quantity);

    HttpResponse::Ok().json(OrderResponse {
        status: "success".into(),
        message: format!("รับคำสั่งซื้อ {} เรียบร้อยแล้ว", item.symbol),
    })
}
