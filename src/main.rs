mod app_state;
mod authentication;
mod contents;
mod enrollments;
mod payments;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, get, web};

use dotenv::dotenv;
use env_logger::Env;
use sqlx::postgres::PgPoolOptions;

/// App State Factory Import
use crate::app_state::app_state_init;

/// ROUTES IMPORTS
use authentication::users::instructors::instructors_controller::{
    create_instructor, get_instructor_by_cognito,
};
use authentication::users::students::student_profiles_controller::{
    create_student_profile, get_student_profile_by_cognito,
};
use authentication::users::students::students_controller::{
    create_student, get_student_by_cognito,
};
use crate::enrollments::enrollments_controller::{
    create_enrollment, get_enrollment, get_enrollment_for_subject_student,
    get_enrollments_by_student, get_enrollments_by_subject, get_not_enrolled,
};
use crate::payments::payments_controller::{create_yoco_checkout, payment_notification_webhook};
use contents::lessons::lesson_controllers::{create_lesson, get_lesson, get_lessons_by_topic};
use contents::subjects::subjects_controller::{
    create_subject, get_subject, get_subjects_by_grade, get_subjects_by_instructor,
    get_subjects_by_term, get_subjects_by_term_and_grade,
};
use contents::topics::topics_controller::{create_topic, get_topic, get_topics_by_subject};

#[get("/health")]
async fn health_check() -> actix_web::Result<String> {
    Ok("The server is running".to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pg_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create PgPool");

    let _ = sqlx::migrate!("./migrations")
        .run(&pg_pool)
        .await
        .map_err(|e| log::error!("{}", e.to_string()));

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .unwrap();
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let frontend_origin =
        std::env::var("FRONTEND_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let payment_webhooks_origin = std::env::var("PAYMENT_WEBHOOKS_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".to_string());

    log::info!("Frontend connecting from {}", &frontend_origin);

    let state = app_state_init(pg_pool);

    HttpServer::new(move || {
        App::new().service(
            web::scope("/api/v1")
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i %r %s %b %T"))
                .service(health_check)
                .wrap(
                    Cors::default()
                        .allowed_origin(&frontend_origin)
                        .allowed_origin(&payment_webhooks_origin)
                        .allowed_methods(["GET", "POST"])
                        .allowed_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
                        .supports_credentials()
                        .max_age(3600),
                )
                .service(
                    web::scope("/payments")
                        .service(create_yoco_checkout)
                        .service(payment_notification_webhook),
                )
                .service(
                    web::scope("/students")
                        .service(get_student_by_cognito)
                        .service(create_student)
                        .service(get_enrollments_by_student)
                        .service(get_not_enrolled),
                )
                .service(
                    web::scope("/student-profiles")
                        .service(get_student_profile_by_cognito)
                        .service(create_student_profile),
                )
                .service(
                    web::scope("/instructors")
                        .service(get_instructor_by_cognito)
                        .service(create_instructor)
                        .service(get_subjects_by_instructor),
                )
                .service(
                    web::scope("/subjects")
                        .service(get_subject)
                        .service(create_subject)
                        .service(get_topics_by_subject)
                        .service(get_enrollments_by_subject),
                )
                .service(
                    web::scope("/enrollments")
                        .service(get_enrollment)
                        .service(create_enrollment)
                        .service(get_enrollment_for_subject_student),
                )
                .service(
                    web::scope("/topics")
                        .service(get_topic)
                        .service(create_topic)
                        .service(get_lessons_by_topic),
                )
                .service(
                    web::scope("/lessons")
                        .service(get_lesson)
                        .service(create_lesson),
                )
                .service(
                    web::scope("/grades")
                        .service(get_subjects_by_grade)
                        .service(get_subjects_by_term_and_grade),
                )
                .service(web::scope("/terms").service(get_subjects_by_term))
                .app_data(state.students.clone())
                .app_data(state.student_profiles.clone())
                .app_data(state.instructors.clone())
                .app_data(state.subjects.clone())
                .app_data(state.topics.clone())
                .app_data(state.lessons.clone())
                .app_data(state.enrollments.clone())
                .app_data(state.checkouts.clone()),
        )
    })
    .workers(4)
    .bind((host, port))?
    .run()
    .await
}
