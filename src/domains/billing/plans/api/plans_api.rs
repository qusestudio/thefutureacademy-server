use crate::configuration::state::AppState;
use crate::domains::billing::plans::models::plan::{ PlanNew};
use crate::infrastructure::middleware::middleware::middleware;
use actix_web::web::{Json, Path};
use actix_web::{HttpResponse, get, post};

#[post("")]
pub async fn create_plan(
    req: actix_web::HttpRequest,
    state: actix_web::web::Data<AppState>,
    payload: Json<PlanNew>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => match claims.custom_role.as_str() {
            "admin" => match state.plans.create_plan(payload.into_inner()).await {
                Ok(plan) => Ok(HttpResponse::Ok().json(Json(plan))),
                Err(error) => {
                    log::error!("Error creating plan: {}", error);
                    Ok(HttpResponse::InternalServerError().json(error.to_string()))
                }
            },
            _ => {
                log::error!("User {} is not authorized to create a plan", claims.sub);
                Ok(HttpResponse::Forbidden()
                    .json("Forbidden: User is not authorized to create a plan"))
            }
        },
        Err(error) => {
            log::error!("Unauthorised: {}", error);
            Ok(HttpResponse::Unauthorized().json(error.to_string()))
        }
    }
}

#[get("/{id}")]
pub async fn get_plan(
    req: actix_web::HttpRequest,
    state: actix_web::web::Data<AppState>,
    id: Path<String>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} getting plan", claims.sub);
            match state.plans.get_plan_by_id(id.into_inner()).await {
                Ok(plan) => Ok(HttpResponse::Ok().json(Json(plan))),
                Err(error) => {
                    log::error!("Error plan by id: {}", error);
                    Ok(HttpResponse::InternalServerError().json(error.to_string()))
                }
            }
        }
        Err(error) => {
            log::error!("Unauthorised: {}", error);
            Ok(HttpResponse::Unauthorized().json(error.to_string()))
        }
    }
}

#[get("/{grade}/plan")]
pub async fn get_plan_by_grade(
    req: actix_web::HttpRequest,
    state: actix_web::web::Data<AppState>,
    grade: Path<i32>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} getting plan", claims.sub);
            match state.plans.get_plan_by_grade(grade.into_inner()).await {
                Ok(plan) => Ok(HttpResponse::Ok().json(Json(plan))),
                Err(error) => {
                    log::error!("Error plan by grade: {}", error);
                    Ok(HttpResponse::InternalServerError().json(error.to_string()))
                }
            }
        }
        Err(error) => {
            log::error!("Unauthorised: {}", error);
            Ok(HttpResponse::Unauthorized().json(error.to_string()))
        }
    }
}

#[get("")]
pub async fn get_plans(
    req: actix_web::HttpRequest,
    state: actix_web::web::Data<AppState>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} all plans", claims.sub);
            match state.plans.get_all_plans().await {
                Ok(plan) => Ok(HttpResponse::Ok().json(Json(plan))),
                Err(error) => {
                    log::error!("Error getting plans: {}", error);
                    Ok(HttpResponse::InternalServerError().json(error.to_string()))
                }
            }
        }
        Err(error) => {
            log::error!("Unauthorised: {}", error);
            Ok(HttpResponse::Unauthorized().json(error.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domains::billing::plans::repo::plan_repo::{MockPlanRepository, PlanRepository};
    use crate::domains::billing::plans::service::plans_service::PlansService;
    use crate::infrastructure::event_bus::event_bus::{Event, EventBus};
    use actix_web::http::StatusCode;
    use actix_web::web::Data;
    use actix_web::{App, test, web};
    use std::sync::Arc;
    use crate::domains::allocations::repo::allocation_repo::MockAllocationRepository;
    use crate::domains::allocations::service::allocations_service::AllocationsService;
    use crate::domains::billing::checkouts::repo::checkout_repository::MockCheckoutRepository;
    use crate::domains::billing::checkouts::service::checkouts_service::CheckoutsService;
    use crate::domains::billing::payments::repo::payment_repo::MockPaymentRepository;
    use crate::domains::billing::payments::service::payments_service::PaymentsService;
    use crate::domains::billing::plans::models::plan::Plan;
    use crate::domains::billing::subscriptions::repo::subscription_repo::MockSubscriptionRepository;
    use crate::domains::billing::subscriptions::service::subscriptions_service::SubscriptionsService;
    use crate::domains::enrollments::repo::enrollment_repo::MockEnrollmentRepository;
    use crate::domains::enrollments::service::enrollments_service::EnrollmentsService;
    use crate::domains::learning::lessons::repo::lesson_repo::MockLessonRepository;
    use crate::domains::learning::lessons::service::lessons_service::LessonsService;
    use crate::domains::learning::subjects::repo::subject_repo::MockSubjectRepository;
    use crate::domains::learning::subjects::service::subjects_service::SubjectsService;
    use crate::domains::learning::topics::repo::topic_repo::MockTopicRepository;
    use crate::domains::learning::topics::service::topics_service::TopicsService;
    use crate::domains::users::admin::repo::admin_repo::MockAdminRepository;
    use crate::domains::users::admin::service::admin_service::AdminsService;
    use crate::domains::users::instructors::repository::instructor_profile_repository::MockInstructorProfileRepository;
    use crate::domains::users::instructors::repository::instructor_repo::MockInstructorRepository;
    use crate::domains::users::instructors::service::instructor_profiles_service::InstructorProfilesService;
    use crate::domains::users::instructors::service::instructors_service::InstructorsService;
    use crate::domains::users::students::repository::student_profile_repo::MockStudentProfileRepository;
    use crate::domains::users::students::repository::student_repo::MockStudentRepository;
    use crate::domains::users::students::service::student_profiles_service::StudentProfilesService;
    use crate::domains::users::students::service::students_service::StudentsService;
    use crate::infrastructure::analytics::repo::analytics_repo::MockAnalyticsRepository;
    use crate::infrastructure::analytics::service::analytics_service::AnalyticsService;
    use crate::infrastructure::channel::events_channel_checker::EventsChannelChecker;
    // ── mock repo ──────────────────────────────────────────────────────────────

    // ── helpers ────────────────────────────────────────────────────────────────
    // TODO: Mock all repos and services
    fn make_test_app_state() -> Data<AppState> {
        let (tx, _rx) = tokio::sync::broadcast::channel::<Event>(16);
        let event_bus: Data<EventBus> = Data::new(tx);

        web::Data::new(AppState {
            analytics: Data::new(AnalyticsService {
                repo: Arc::new(MockAnalyticsRepository::new()),
                event_bus: event_bus.clone(),
            }),
            admins: Data::new(AdminsService {
                repo: Arc::new(MockAdminRepository::new()),
                event_bus: event_bus.clone(),
            }),
            health_check_service: Data::new(EventsChannelChecker {
                event_bus: event_bus.clone(),
            }),
            students: Data::new(StudentsService {
                repo: Arc::new(MockStudentRepository::new()),
                event_bus: event_bus.clone(),
            }),
            student_profiles: Data::new(StudentProfilesService {
                repo: Arc::new(MockStudentProfileRepository::new()),
                event_bus: event_bus.clone(),
            }),
            instructors: Data::new(InstructorsService {
                repo: Arc::new(MockInstructorRepository::new()),
                event_bus: event_bus.clone(),
            }),
            instructor_profiles: Data::new(InstructorProfilesService {
                repo: Arc::new(MockInstructorProfileRepository::new()),
                event_bus: event_bus.clone(),
            }),
            subjects: Data::new(SubjectsService {
                repo: Arc::new(MockSubjectRepository::new()),
                event_bus: event_bus.clone(),
            }),
            topics: Data::new(TopicsService {
                repo: Arc::new(MockTopicRepository::new()),
                event_bus: event_bus.clone(),
            }),
            lessons: Data::new(LessonsService {
                repo: Arc::new(MockLessonRepository::new()),
                event_bus: event_bus.clone(),
            }),
            enrollments: Data::new(EnrollmentsService {
                repo: Arc::new(MockEnrollmentRepository::new()),
                event_bus: event_bus.clone(),
            }),
            allocations: Data::new(AllocationsService {
                repo: Arc::new(MockAllocationRepository::new()),
                event_bus: event_bus.clone(),
            }),
            checkouts: Data::new(CheckoutsService {
                repo: Arc::new(MockCheckoutRepository::new()),
                event_bus: event_bus.clone(),
            }),
            payments: Data::new(PaymentsService {
                repo: Arc::new(MockPaymentRepository::new()),
                event_bus: event_bus.clone(),
            }),
            plans: Data::new(PlansService {
                repo: Arc::new(MockPlanRepository::new()),
                event_bus: event_bus.clone(),
            }),
            subscriptions: Data::new(SubscriptionsService {
                repo: Arc::new(MockSubscriptionRepository::new()),
                event_bus: event_bus.clone(),
            }),
        })
    }

    fn admin_token() -> String {
        "test.admin.test-user-id".to_string()
    }

    fn user_token() -> String {
        "test.user.test-user-id".to_string()
    }

    fn auth_header(token: &str) -> (&'static str, String) {
        ("Authorization", format!("Bearer {}", token))
    }

    // Rather than constructing the entire AppState, we only register
    // the PlansService directly since that's all these handlers need
    macro_rules! plans_app {
        () => {
            test::init_service(
                App::new().app_data(make_test_app_state()).service(
                    web::scope("/api/v1").service(
                        web::scope("/plans")
                            .service(create_plan)
                            .service(get_plan)
                            .service(get_plans),
                    ),
                ),
            )
            .await
        };
    }

    // ── create_plan ────────────────────────────────────────────────────────────

    #[actix_web::test]
    async fn create_plan_returns_200_for_admin() {
        let app = plans_app!();
        let payload = serde_json::json!({
            "name": "Basic Plan",
            "price": 999,
            "duration": 30,
            "is_active": true
        });
        let (header_name, header_value) = auth_header(&admin_token());

        let req = test::TestRequest::post()
            .uri("/api/v1/plans")
            .insert_header((header_name, header_value))
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn create_plan_returns_403_for_non_admin() {
        let app = plans_app!();
        let payload = serde_json::json!({
            "name": "Basic Plan",
            "price": 999,
            "duration": 30,
            "is_active": true
        });
        let (header_name, header_value) = auth_header(&user_token());

        let req = test::TestRequest::post()
            .uri("/api/v1/plans")
            .insert_header((header_name, header_value))
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }

    #[actix_web::test]
    async fn create_plan_returns_401_without_token() {
        let app = plans_app!();
        let payload = serde_json::json!({
            "name": "Basic Plan",
            "price": 999,
            "duration": 30,
            "is_active": true
        });

        let req = test::TestRequest::post()
            .uri("/api/v1/plans")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    // ── get_plan ───────────────────────────────────────────────────────────────

    #[actix_web::test]
    async fn get_plan_returns_200_with_valid_token() {
        let app = plans_app!();
        let (header_name, header_value) = auth_header(&user_token());

        let req = test::TestRequest::get()
            .uri("/api/v1/plans/mock-id-1")
            .insert_header((header_name, header_value))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn get_plan_returns_correct_body() {
        let app = plans_app!();
        let (header_name, header_value) = auth_header(&user_token());

        let req = test::TestRequest::get()
            .uri("/api/v1/plans/mock-id-1")
            .insert_header((header_name, header_value))
            .to_request();

        let plan: Plan = test::call_and_read_body_json(&app, req).await;
        assert_eq!(plan.id, "mock-id-1");
        assert_eq!(plan.name, "Mock Plan");
    }

    #[actix_web::test]
    async fn get_plan_returns_401_without_token() {
        let app = plans_app!();

        let req = test::TestRequest::get()
            .uri("/api/v1/plans/mock-id-1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    // ── get_plans ──────────────────────────────────────────────────────────────

    #[actix_web::test]
    async fn get_plans_returns_200_with_valid_token() {
        let app = plans_app!();
        let (header_name, header_value) = auth_header(&user_token());

        let req = test::TestRequest::get()
            .uri("/api/v1/plans")
            .insert_header((header_name, header_value))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn get_plans_returns_correct_count() {
        let app = plans_app!();
        let (header_name, header_value) = auth_header(&user_token());

        let req = test::TestRequest::get()
            .uri("/api/v1/plans")
            .insert_header((header_name, header_value))
            .to_request();

        let plans: Vec<Plan> = test::call_and_read_body_json(&app, req).await;
        assert_eq!(plans.len(), 2);
    }

    #[actix_web::test]
    async fn get_plans_returns_401_without_token() {
        let app = plans_app!();

        let req = test::TestRequest::get().uri("/api/v1/plans").to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}
