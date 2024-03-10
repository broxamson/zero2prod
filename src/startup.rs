use std::net::TcpListener;

use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use sqlx::PgPool;
use actix_web::middleware::Logger;
use crate::routes::health_check::health_check;
use crate::routes::subcriptions::subscribe;

pub fn run(address: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(address)?
    .run();
    Ok(server)
}
