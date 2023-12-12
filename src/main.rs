// rocketはリクエストを受け付け、処理を実施した後にレスポンスを返すWebフレームワーク

#[macro_use] extern crate rocket;

#[get("/hello")]                // <-- ルート
fn index() -> &'static str {    // <-- ハンドラ
    "Hello, Rocket"
}


#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/test", routes![index])
}