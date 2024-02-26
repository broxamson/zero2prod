use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::{Pool, Postgres};

use crate::routes::health_check::health_check;
use crate::routes::subcriptions::subscribe;

pub fn run(
    address: TcpListener,
    connection: Pool<Postgres>,
) -> actix_web::Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(address)?
    .run();
    Ok(server)
}
