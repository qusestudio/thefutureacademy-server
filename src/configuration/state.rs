use actix_web::web::Data;
use sqlx::PgPool;
use std::sync::Arc;
use crate::configuration::events::event_handlers_init;
use crate::domains::billing::checkouts::repo::postgres_checkout_repo::PostgresCheckoutRepository;
use crate::domains::billing::checkouts::service::checkouts_service::CheckoutsService;
use crate::domains::billing::payments::repo::postgres_payment_repo::PostgresPaymentRepo;
use crate::domains::billing::payments::service::payments_service::PaymentsService;
use crate::domains::billing::subscription::repo::subscription_repository_pg::PostgresSubscriptionRepo;
use crate::domains::billing::subscription::service::subscriptions_service::SubscriptionsService;
use crate::infrastructure::channel::events_channel_checker::EventsChannelChecker;
use crate::domains::enrollments::repo::postgres_enrollment_repo::PostgresEnrollmentRepo;
use crate::domains::enrollments::service::enrollments_service::EnrollmentsService;
use crate::domains::learning::lessons::repo::postgres_lesson_repo::PostgresLessonRepo;
use crate::domains::learning::lessons::service::lessons_service::LessonsService;
use crate::domains::learning::subjects::repo::postgres_subject_repo::PostgresSubjectRepo;
use crate::domains::learning::subjects::service::subjects_service::SubjectsService;
use crate::domains::learning::topics::repo::postgres_topic_repo::PostgresTopicRepo;
use crate::domains::learning::topics::service::topics_service::TopicsService;
use crate::domains::users::admin::repo::postgres_admin_repo::PostgresAdminRepo;
use crate::domains::users::admin::service::admin_service::AdminsService;
use crate::domains::users::instructors::repository::postgres_instructor_profile_repo::PostgresInstructorProfileRepo;
use crate::domains::users::instructors::repository::postgres_instructor_repo::PostgresInstructorRepo;
use crate::domains::users::instructors::service::instructor_profiles_service::InstructorProfilesService;
use crate::domains::users::instructors::service::instructors_service::InstructorsService;
use crate::domains::users::students::repository::pg_student_profile_repo::PGStudentProfileRepo;
use crate::domains::users::students::repository::postgres_student_repo::PostgresStudentRepo;
use crate::domains::users::students::service::student_profiles_service::StudentProfilesService;
use crate::domains::users::students::service::students_service::StudentsService;
use crate::infrastructure::database::postgres::{init_pool, run_migrations};
use crate::infrastructure::event_bus::event_bus::{EventBus, init_bus};
use crate::domains::allocations::repo::postgres_allocation_repo::PostgresAllocationRepo;
use crate::domains::allocations::service::allocations_service::AllocationsService;
use crate::infrastructure::analytics::repo::postgres_analytics_repo::{PostgresAnalyticsRepository};
use crate::infrastructure::analytics::service::analytics_service::AnalyticsService;

#[derive(Clone)]
pub struct AppState {
    pub analytics: Data<AnalyticsService>,
    pub admins: Data<AdminsService>,
    pub health_check_service: Data<EventsChannelChecker>,
    pub students: Data<StudentsService>,
    pub student_profiles: Data<StudentProfilesService>,
    pub instructors: Data<InstructorsService>,
    pub instructor_profiles: Data<InstructorProfilesService>,
    pub subjects: Data<SubjectsService>,
    pub topics: Data<TopicsService>,
    pub lessons: Data<LessonsService>,
    pub enrollments: Data<EnrollmentsService>,
    pub allocations: Data<AllocationsService>,
    pub checkouts: Data<CheckoutsService>,
    pub payments: Data<PaymentsService>,
    pub subscriptions: Data<SubscriptionsService>,
}

pub fn app_state(pg_pool: PgPool, event_bus: Data<EventBus>) -> AppState {
    AppState {
        analytics: Data::new(AnalyticsService {
            repo: Arc::new(PostgresAnalyticsRepository{
                pg_pool: pg_pool.clone()
            }),
            event_bus: event_bus.clone(),
        }),
        admins: Data::new(AdminsService {
            repo: Arc::new(PostgresAdminRepo {
                pg_pool: pg_pool.clone(),
            }),
            event_bus: event_bus.clone(),
        }),
        health_check_service: Data::new(EventsChannelChecker {
            event_bus: event_bus.clone(),
        }),
        students: Data::new(StudentsService {
            repo: Arc::new(PostgresStudentRepo {
                pg_pool: pg_pool.clone(),
            }),
            event_bus: event_bus.clone(),
        }),
        student_profiles: Data::new(StudentProfilesService {
            repo: Arc::new(PGStudentProfileRepo {
                pg_pool: pg_pool.clone(),
            }),
            event_bus: event_bus.clone(),
        }),
        instructors: Data::new(InstructorsService {
            repo: Arc::new(PostgresInstructorRepo {
                pg_pool: pg_pool.clone(),
            }),
            event_bus: event_bus.clone(),
        }),
        instructor_profiles: Data::new(InstructorProfilesService {
            repo: Arc::new(PostgresInstructorProfileRepo {
                pg_pool: pg_pool.clone(),
            }),
            event_bus: event_bus.clone(),
        }),
        subjects: Data::new(SubjectsService {
            repo: Arc::new(PostgresSubjectRepo {
                pg_pool: pg_pool.clone(),
            }),
            event_bus: event_bus.clone(),
        }),
        topics: Data::new(TopicsService {
            repo: Arc::new(PostgresTopicRepo {
                pg_pool: pg_pool.clone(),
            }),
            event_bus: event_bus.clone(),
        }),
        lessons: Data::new(LessonsService {
            repo: Arc::new(PostgresLessonRepo {
                pg_pool: pg_pool.clone(),
            }),
            event_bus: event_bus.clone(),
        }),
        enrollments: Data::new(EnrollmentsService {
            repo: Arc::new(PostgresEnrollmentRepo {
                pg_pool: pg_pool.clone(),
            }),
            event_bus: event_bus.clone(),
        }),
        allocations: Data::new(AllocationsService {
            repo: Arc::new(PostgresAllocationRepo {
                pg_pool: pg_pool.clone()
            }),
            event_bus: event_bus.clone(),
        }),
        checkouts: Data::new(CheckoutsService {
            repo: Arc::new(PostgresCheckoutRepository {
                pg_pool: pg_pool.clone(),
            }),
            event_bus: event_bus.clone(),
        }),
        payments: Data::new(PaymentsService {
            repo: Arc::new(PostgresPaymentRepo {
                pg_pool: pg_pool.clone(),
            }),
            event_bus: event_bus.clone(),
        }),
        subscriptions: Data::new(SubscriptionsService {
            repo: Arc::new(PostgresSubscriptionRepo {
                pg_pool: pg_pool.clone(),
            }),
            event_bus: event_bus.clone(),
        }),
    }
}

pub async fn build_state() -> AppState {
    // INITIALIZE POSTGRES CONNECTION POOL
    log::info!("Building state...");
    let pg_pool = init_pool().await;
    // RUN MIGRATIONS
    run_migrations(&pg_pool).await;
    // INITIALIZE EVENT BUS
    let tx = init_bus();
    let event_bus = Data::new(tx.clone());
    // INITIALIZE APP STATE
    let state = app_state(pg_pool, event_bus);
    event_handlers_init(tx, Data::new(state.clone())).await;
    state
}
