mod middleware;
mod users;

use crate::users::students::repository::postgres_student_repo::PostgresStudentRepo;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, get, web};
use dotenv::dotenv;
use env_logger::Env;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

/* ROUTES IMPORTS */
use crate::users::students::students_controller::{create_student, get_student_by_cognito};
use crate::users::instructors::instructors_controller::{create_instructor, get_instructor_by_cognito};
use crate::users::instructors::instructors_state::InstructorsState;
use crate::users::instructors::repository::postgres_instructor_repo::PostgresInstructorRepo;
use crate::users::students::students_state::StudentsState;

#[get("/health")]
async fn health_check() -> actix_web::Result<String> {
    Ok("The server is running".to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let pool = PgPool::connect(&database_url).await.expect("");
    let pg_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create PgPool");

    sqlx::migrate!("./migrations")
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .unwrap();
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let frontend_origin =
        std::env::var("FRONTEND_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".to_string());


    let students_state = web::Data::new(StudentsState {
        repo: Arc::new(PostgresStudentRepo { pg_pool: pg_pool.clone() }),
    });

    let instructors_state = web::Data::new(InstructorsState {
        repo: Arc::new(PostgresInstructorRepo { pg_pool: pg_pool.clone() }),
    });

    HttpServer::new(move || {
        App::new().service(
            web::scope("/api/v1")
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i %r %s %b %T"))
                .service(health_check)
                .service(
                    web::scope("/students")
                        .wrap(
                            Cors::default()
                                .allowed_origin(&frontend_origin)
                                .allowed_methods(["GET", "POST"])
                                .allowed_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
                                .supports_credentials()
                                .max_age(3600),
                        )
                        .service(get_student_by_cognito)
                        .service(create_student)
                        .app_data(students_state.clone()),
                )
                .service(
                    web::scope("/instructors")
                        .wrap(
                            Cors::default()
                                .allowed_origin(&frontend_origin)
                                .allowed_methods(["GET", "POST"])
                                .allowed_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
                                .supports_credentials()
                                .max_age(3600),
                        )
                        .service(get_instructor_by_cognito)
                        .service(create_instructor)
                        .app_data(instructors_state.clone()),
                ),
        )
    })
    .workers(4)
    .bind((host, port))?
    .run()
    .await
}
