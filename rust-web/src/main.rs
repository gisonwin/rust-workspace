mod mutex;
mod rocket_sample;

use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
// async fn index() -> impl Responder{
//     "Hello Actix_WEB"
// }

struct AppState{
    app_name:String,
}
#[get("/")]
async fn index(data:web::Data<AppState>) -> String{
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .service(web::scope("/app").route("/index.html",web::get().to(index)))
            .app_data(web::Data::new(AppState{app_name:String::from("Actix web")})).service(index)
            // .service(hello)
            // .service(echo)
            // .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8890))?.run().await
}
