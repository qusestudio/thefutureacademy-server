use actix_web::{get, web, App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;

#[get("/test")]
async fn health_check() -> actix_web::Result<String> {
    Ok("The server is running".to_string())
}

#[actix_web::main]
async fn main() ->std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new().service(
            web::scope("/api/v1")
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i %r %s %b %T"))
                .service(health_check)
        )
    })
        .workers(4)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
