mod routes;
mod startup;
pub mod configuration;

use crate::routes::health_check::health_check;
use crate::routes::subcriptions::subscribe;
use actix_web::dev::Server;
use actix_web::*;
use std::net::TcpListener;

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
