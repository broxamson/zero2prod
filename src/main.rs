use std::io;

use sqlx::PgPool;
use tracing_log::LogTracer;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // Fetch configuration once
    LogTracer::init().expect("Failed to Set logger");
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");

    // Connect to the PostgreSQL database
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    // Bind the server address
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = std::net::TcpListener::bind(address)?;

    // Run the server
    run(listener, connection)?.await
}
