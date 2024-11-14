mod auth;
use auth::auth::{log_in, start_session};
use actix_web::{HttpServer, App};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
                .service(log_in)
                .service(start_session)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}