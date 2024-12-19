// use std::fmt::Debug;
// use actix_web::{web, App, HttpServer, guard, HttpResponse, Responder};
// use std::sync::Mutex;
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
//
// struct AppStateWithCounter {
//     counter: Mutex<i32>,
// }
//
// async fn index(data: web::Data<AppStateWithCounter>) -> String {
//     let mut counter = data.counter.lock().unwrap();
//     *counter += 1;
//     format!("Request nubmer:{counter}")
// }
//
// fn scoped_config(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::resource("/test").route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
//             .route(web::head().to(HttpResponse::MethodNotAllowd)),
//     );
// }
//
// fn config(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::resource("/app")
//             .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
//             .route(web::head().to(HttpResponse::MethodNotAllowed)),
//     );
// }
//
// struct MyInfo {
//     id: i32,
//     username: String,
// }
//
// async fn index2(path: web::Path<(String, String)>, json: web::Json<MyInfo>) -> impl Responder {
//     let path = path.into_inner();
//     format!("{},{},{},{}", path.0, path.1, json.id, json.username)
// }
//
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let counter = web::Data::new(AppStateWithCounter { counter: Mutex::new(0) });
//
//     let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
//     builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
//     builder.set_certificate_chain_file("cert.pem").unwrap();
//
//
//     HttpServer::new(move || {
//         App::new()
//             .configure(config)
//             .service(web::scope("/api").configure(scoped_config))
//             .route("/", web::get().to(|| async { HttpResponse::Ok().body("/") }))
//         // .service(web::scope("/").guard(guard::Host("www.rust-lang.org")).route("",web::to(|| async {HttpResponse::Ok().body("www")})),)
//         // .service(web::scope("/").guard(guard::Host("users.rust-lang.org")).route("",web::to(|| async {HttpResponse::Ok().body("user")})),  )
//         // .route("/",web::to(HttpResponse::Ok))
//         // .app_data(counter.clone()).route("/", web::get().to(index))
//     })
//         .workers(8) //指定多少个 HTTP worker
//         // .bind(("127.0.0.1", 8890))?
//         .bind_openssl("127.0.0.1:8890", builder)?
//         .run().await
// }