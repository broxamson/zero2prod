use actix_web::dev::Server;
use actix_web::*;
use std::net::TcpListener;
async fn subscribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(address: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(address)?
    .run();
    Ok(server)
}
