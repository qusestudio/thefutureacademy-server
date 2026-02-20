use crate::authentication::users::instructors::instructors_state::InstructorsState;
use crate::authentication::users::instructors::repository::postgres_instructor_repo::PostgresInstructorRepo;
use crate::authentication::users::students::repository::pg_student_profile_repo::PGStudentProfileRepo;
use crate::authentication::users::students::repository::postgres_student_repo::PostgresStudentRepo;
use crate::authentication::users::students::students_state::{StudentProfilesState, StudentsState};
use crate::contents::lessons::lessons_state::LessonsState;
use crate::contents::lessons::repo::postgres_lesson_repo::PostgresLessonRepo;
use crate::contents::subjects::repo::postgres_subject_repo::PostgresSubjectRepo;
use crate::contents::subjects::subjects_state::SubjectsState;
use crate::contents::topics::repo::postgres_topic_repo::PostgresTopicRepo;
use crate::contents::topics::topics_state::TopicsState;
use crate::enrollments::enrollments_state::EnrollmentsState;
use crate::enrollments::repo::postgres_enrollment_repo::PostgresEnrollmentRepo;
use crate::payments::checkouts_state::CheckoutsState;
use crate::payments::repo::postgres_checkout_repo::PostgresCheckoutRepository;
use actix_web::web;
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    pub students: web::Data<StudentsState>,
    pub student_profiles: web::Data<StudentProfilesState>,
    pub instructors: web::Data<InstructorsState>,
    pub subjects: web::Data<SubjectsState>,
    pub topics: web::Data<TopicsState>,
    pub lessons: web::Data<LessonsState>,
    pub enrollments: web::Data<EnrollmentsState>,
    pub checkouts: web::Data<CheckoutsState>,
}

pub fn app_state_init(pg_pool: PgPool) -> AppState {
    AppState {
        students: web::Data::new(StudentsState {
            repo: Arc::new(PostgresStudentRepo {
                pg_pool: pg_pool.clone(),
            }),
        }),
        student_profiles: web::Data::new(StudentProfilesState {
            repo: Arc::new(PGStudentProfileRepo {
                pg_pool: pg_pool.clone(),
            }),
        }),
        instructors: web::Data::new(InstructorsState {
            repo: Arc::new(PostgresInstructorRepo {
                pg_pool: pg_pool.clone(),
            }),
        }),
        subjects: web::Data::new(SubjectsState {
            repo: Arc::new(PostgresSubjectRepo {
                pg_pool: pg_pool.clone(),
            }),
        }),
        topics: web::Data::new(TopicsState {
            repo: Arc::new(PostgresTopicRepo {
                pg_pool: pg_pool.clone(),
            }),
        }),
        lessons: web::Data::new(LessonsState {
            repo: Arc::new(PostgresLessonRepo {
                pg_pool: pg_pool.clone(),
            }),
        }),
        enrollments: web::Data::new(EnrollmentsState {
            repo: Arc::new(PostgresEnrollmentRepo {
                pg_pool: pg_pool.clone(),
            }),
        }),
        checkouts: web::Data::new(CheckoutsState {
            repo: Arc::new(PostgresCheckoutRepository {
                pg_pool: pg_pool.clone(),
            }),
        }),
    }
}
