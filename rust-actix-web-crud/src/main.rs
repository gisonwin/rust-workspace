mod model;
mod response;
mod handler;

use actix_web::middleware::Logger;
use actix_web::{App, http::header, HttpServer, web};
use serde::Serialize;
use actix_cors::Cors;
use model::AppState;

/**
 *  @Description
 *  @author <a href="mailto:gisonwin@qq.com">GiSon.Win</a>
 *  @Date 2023/11/9 15:49
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let todo_db
        = AppState::init();
    let app_data = web::Data::new(todo_db);

    println!("Server started successfully!");

    HttpServer::new(move || {
        //add cors
        let cors = Cors::default().allowed_origin("http://localhost:3000").allowed_origin("http://localhost:3000/").allowed_methods(vec!["GET", "POST"])
            .allowed_header(vec![header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
            .supports_credentials();
        App::new().app_data(app_data.clone())
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
        .bind(("127.0.0.1", 8900))?.run().await
}
