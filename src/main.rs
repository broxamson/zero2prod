use std::io::Error;
use zero2prod::run;
use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> actix_web::Result<(), Error> {
    let configuration = get_configuration().expect("Failed To Read configuration");
    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = std::net::TcpListener::bind(address)?;
    run(listener)?.await
}
