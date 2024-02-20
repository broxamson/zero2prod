use std::io::Error;
use zero2prod::run;
#[tokio::main]
async fn main() -> actix_web::Result<(), Error> {
    let listener = std::net::TcpListener::bind("0.0.0.0:8000")?;
    run(listener)?.await
}
