use actix_web::{web::Data, App, HttpServer};
use tera::Tera;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Loading templates...");
    let tera = Data::new(Tera::new("templates/**").unwrap());

    println!("Starting server...");
    HttpServer::new(move || {
        App::new()
            .app_data(tera.clone())
            .service(routes::index::index)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
