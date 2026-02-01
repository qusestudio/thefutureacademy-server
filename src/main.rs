mod users;

use std::sync::Arc;
use actix_web::{get, web, App, HttpServer};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use env_logger::Env;
use sqlx::postgres::PgPoolOptions;
use crate::users::students::repository::postgres_student_repo::PostgresStudentRepo;
/* ROUTES IMPORTS */
use crate::users::students::students_controller::{create_student, get_student_by_cognito};
use crate::users::students::students_state::StudentsState;

#[get("/health")]
async fn health_check() -> actix_web::Result<String> {
    Ok("The server is running".to_string())
}

#[actix_web::main]
async fn main() ->std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let pool = PgPool::connect(&database_url).await.expect("");
    let pg_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create PgPool");

    let _ = sqlx::migrate!().run(&pg_pool).await.map_err(|e| format!("Oh no! Migrations failed :( {e}"));


    let port = std::env::var("PORT").unwrap_or_else(|_| {"8000".to_string()}).parse::<u16>().unwrap();
    let host = std::env::var("HOST").unwrap_or_else(|_| {"0.0.0.0".to_string()});

    let students_state = web::Data::new( StudentsState {
        repo: Arc::new(
            PostgresStudentRepo {
                pg_pool
            }
        )
    } );

    HttpServer::new( move || {
        App::new().service(
            web::scope("/api/v1")
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i %r %s %b %T"))
                .service(health_check)
                .service(
                    web::scope("/students")
                        .service(get_student_by_cognito)
                        .service(create_student)
                        .app_data(students_state.clone()),
                )
        )
    })
        .workers(4)
        .bind((host, port))?
        .run()
        .await
}
