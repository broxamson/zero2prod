use std::io::Error;

use sqlx::{Connection, PgConnection};

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> actix_web::Result<(), Error> {
    let connection = PgConnection::connect(
        &get_configuration()
            .expect("Failed to read configuration")
            .database
            .connection_string(),
    )
    .await
    .expect("Failed to connect to Postgres");
    let configuration = get_configuration().expect("Failed To Read configuration");
    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = std::net::TcpListener::bind(address)?;
    run(listener, connection)?.await
}
