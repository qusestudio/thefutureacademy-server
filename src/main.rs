use actix_web::{get, web, App, HttpServer};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use env_logger::Env;

#[get("/health")]
async fn health_check() -> actix_web::Result<String> {
    Ok("The server is running".to_string())
}

#[actix_web::main]
async fn main() ->std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| {"8000".to_string()}).parse::<u16>().unwrap();
    let host = std::env::var("HOST").unwrap_or_else(|_| {"localhost".to_string()});

    HttpServer::new(|| {
        App::new().service(
            web::scope("/api/v1")
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i %r %s %b %T"))
                .service(health_check)
        )
    })
        .workers(4)
        .bind((host, port))?
        .run()
        .await
}
