use actix_web::web;
use crate::domains::allocations::api::allocations_api::{get_instructor_allocations, set_teaching_allocation};
use crate::domains::billing::payments::api::payments_api::{create_yoco_checkout, get_checkout_by_student, payment_notification_webhook};
use crate::infrastructure::channel::health_api::send_test_event;
use crate::domains::enrollments::api::enrollments_api::{create_enrollment, get_enrollment, get_enrollment_for_subject_student, get_enrollments_by_student, get_enrollments_by_subject, get_not_enrolled};
use crate::domains::learning::lessons::api::lessons_api::{create_lesson, get_lesson, get_lessons_by_topic};
use crate::domains::learning::subjects::api::subjects_api::{create_subject, delete_subjects, get_all_subjects, get_subject, get_subjects_by_grade, get_subjects_by_term, get_subjects_by_term_and_grade};
use crate::domains::learning::topics::api::topics_api::{create_topic, delete_topics, get_topic, get_topics_by_subject};
use crate::domains::users::admin::api::admins_api::{create_admin, get_admin_by_cognito};
use crate::domains::users::instructors::api::instructor_profiles_api::{create_instructor_profile, get_instructor_profile};
use crate::domains::users::instructors::api::instructors_api::{create_instructor, get_instructor_by_cognito};
use crate::domains::users::students::api::student_profiles_api::{create_student_profile, get_student_profile_by_cognito};
use crate::domains::users::students::api::students_api::{create_student, get_student_by_cognito};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(send_test_event)
            .service(
                web::scope("/payments")
                    .service(create_yoco_checkout)
                    .service(payment_notification_webhook),
            )
            .service(
                web::scope("/admins")
                    .service(get_admin_by_cognito)
                    .service(create_admin)
            )
            .service(
                web::scope("/students")
                    .service(get_student_by_cognito)
                    .service(create_student)
                    .service(get_enrollments_by_student)
                    .service(get_not_enrolled)
                    .service(get_checkout_by_student),
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
                    .service(get_instructor_allocations)
            )
            .service(
                web::scope("/instructor-profiles")
                    .service(get_instructor_profile)
                    .service(create_instructor_profile)
            )
            .service(
                web::scope("/subjects")
                    .service(get_subject)
                    .service(create_subject)
                    .service(get_topics_by_subject)
                    .service(get_all_subjects)
                    .service(get_enrollments_by_subject)
                    .service(delete_subjects),
            )
            .service(
                web::scope("/enrollments")
                    .service(get_enrollment)
                    .service(create_enrollment)
                    .service(get_enrollment_for_subject_student),
            )
            .service(
                web::scope("/allocations")
                    .service(set_teaching_allocation),
            )
            .service(
                web::scope("/topics")
                    .service(get_topic)
                    .service(create_topic)
                    .service(get_lessons_by_topic)
                    .service(delete_topics),
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
    );
}