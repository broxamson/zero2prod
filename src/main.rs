use std::io::Error;
use actix_web::dev::Server;
use zero2prod::run;
#[tokio::main]
async fn main() -> actix_web::Result<(), Error> {
    run()?.await
    }


