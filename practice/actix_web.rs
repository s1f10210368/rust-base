// actix_webは非同期処理をするサーバの機能を持っている
// 際ぢあの特徴はアクターモデルを使用していること

use actix_web::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}

#[actix_web::main]  // <-- この書き方をすると main を非同期で実行できる
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}