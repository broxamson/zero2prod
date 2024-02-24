use std::net::TcpListener;

use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use sqlx::PgConnection;

use crate::routes::health_check::health_check;
use crate::routes::subcriptions::subscribe;

pub fn run(
    address: TcpListener,
    connection: PgConnection,
) -> actix_web::Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
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
