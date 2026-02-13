mod middleware;
mod subjects;
mod users;
mod topics;
mod lessons;
mod enrollments;

use crate::subjects::repo::postgres_subject_repo::PostgresSubjectRepo;
use crate::subjects::subjects_controller::{
    create_subject, get_subject, get_subjects_by_grade, get_subjects_by_instructor,
    get_subjects_by_term, get_subjects_by_term_and_grade,
};
use crate::subjects::subjects_state::SubjectsState;
use crate::users::students::repository::postgres_student_repo::PostgresStudentRepo;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, get, web};
use dotenv::dotenv;
use env_logger::Env;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use crate::enrollments::enrollments_controller::{create_enrollment, get_enrollment, get_enrollment_for_subject_student, get_enrollments_by_student, get_enrollments_by_subject, get_not_enrolled};
use crate::enrollments::enrollments_state::EnrollmentsState;
use crate::enrollments::repo::postgres_enrollment_repo::PostgresEnrollmentRepo;
use crate::lessons::lesson_controllers::{create_lesson, get_lesson, get_lessons_by_topic};
use crate::lessons::lessons_state::LessonsState;
use crate::lessons::repo::postgres_lesson_repo::PostgresLessonRepo;
use crate::topics::repo::postgres_topic_repo::PostgresTopicRepo;
use crate::topics::topics_controller::{create_topic, get_topic, get_topics_by_subject};
use crate::topics::topics_state::TopicsState;
/* ROUTES IMPORTS */
use crate::users::instructors::instructors_controller::{
    create_instructor, get_instructor_by_cognito,
};
use crate::users::instructors::instructors_state::InstructorsState;
use crate::users::instructors::repository::postgres_instructor_repo::PostgresInstructorRepo;
use crate::users::students::students_controller::{create_student, get_student_by_cognito};
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

    let pg_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create PgPool");

    let _ = sqlx::migrate!("./migrations")
        .run(&pg_pool)
        .await
        .map_err(|e| {
        log::error!("{}", e.to_string())
    });

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .unwrap();
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let frontend_origin =
        std::env::var("FRONTEND_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".to_string());

    log::info!("Frontend connecting from {}", &frontend_origin);

    let students_state = web::Data::new(StudentsState {
        repo: Arc::new(PostgresStudentRepo {
            pg_pool: pg_pool.clone(),
        }),
    });

    let instructors_state = web::Data::new(InstructorsState {
        repo: Arc::new(PostgresInstructorRepo {
            pg_pool: pg_pool.clone(),
        }),
    });

    let subjects_state = web::Data::new(SubjectsState {
        repo: Arc::new(PostgresSubjectRepo {
            pg_pool: pg_pool.clone(),
        }),
    });

    let topics_state = web::Data::new(TopicsState {
        repo: Arc::new(PostgresTopicRepo {
            pg_pool: pg_pool.clone()
        })
    });

    let lessons_state = web::Data::new( LessonsState {
        repo: Arc::new( PostgresLessonRepo {
            pg_pool: pg_pool.clone()
        })
    });
    
    let enrollments_state = web::Data::new( EnrollmentsState {
        repo: Arc::new(PostgresEnrollmentRepo {
            pg_pool: pg_pool.clone()
        })
    });


    HttpServer::new(move || {
        App::new().service(
            web::scope("/api/v1")
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i %r %s %b %T"))
                .service(health_check)
                .wrap(
                    Cors::default()
                        .allowed_origin(&frontend_origin)
                        .allowed_methods(["GET", "POST"])
                        .allowed_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
                        .supports_credentials()
                        .max_age(3600),
                )
                .service(
                    web::scope("/students")
                        .service(get_student_by_cognito)
                        .service(create_student)
                        .service(get_enrollments_by_student)
                        .service(get_not_enrolled),
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
                        .service(get_enrollment_for_subject_student)
                )
                .service(
                    web::scope("/topics")
                        .service(get_topic)
                        .service(create_topic)
                        .service(get_lessons_by_topic)
                ).service(
                    web::scope("/lessons")
                        .service(get_lesson)
                        .service(create_lesson)
                )
                .service(
                    web::scope("/grades")
                        .service(get_subjects_by_grade)
                        .service(get_subjects_by_term_and_grade),
                )
                .service(
                    web::scope("/terms")
                        .service(get_subjects_by_term)
                )
                .app_data(students_state.clone())
                .app_data(instructors_state.clone())
                .app_data(subjects_state.clone())
                .app_data(topics_state.clone())
                .app_data(lessons_state.clone())
                .app_data(enrollments_state.clone()),
        )
    })
    .workers(4)
    .bind((host, port))?
    .run()
    .await
}
