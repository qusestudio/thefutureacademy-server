use crate::enrollments::enrollments_state::EnrollmentsState;
use crate::enrollments::models::enrollment::EnrollmentNew;
use crate::middleware::middleware::middleware;
use actix_web::web::{Json, Path};
use actix_web::{HttpRequest, HttpResponse, get, post};
use serde::Deserialize;

#[get("/{subject_id}/enrollments")]
pub async fn get_enrollments_by_subject(
    state: actix_web::web::Data<EnrollmentsState>,
    req: HttpRequest,
    subject_id: Json<String>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!(
                "User {} getting Enrollments with subject id {}",
                claims.sub,
                &subject_id.clone()
            );

            match state
                .repo
                .db_get_enrollments_by_subject(subject_id.into_inner().as_str())
                .await
            {
                Ok(student_enrollments) => Ok(HttpResponse::Ok().json(Json(student_enrollments))),
                Err(e) => Ok(HttpResponse::NotFound().json(e.to_string())),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

#[get("/{student_id}/enrollments")]
pub async fn get_enrollments_by_student(
    state: actix_web::web::Data<EnrollmentsState>,
    req: HttpRequest,
    student_id: Path<String>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!(
                "User {} getting Enrollments with student id {}",
                claims.sub,
                &student_id.clone()
            );

            match state
                .repo
                .db_get_enrollments_by_student(student_id.as_str())
                .await
            {
                Ok(enrollments) => Ok(HttpResponse::Ok().json(Json(enrollments))),
                Err(e) => Ok(HttpResponse::NotFound().json(e.to_string())),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

#[derive(Deserialize)]
pub struct AvailableSubjectReq {
    pub student_id: String,
    pub grade: i32,
}

#[get("/{student_id}/grade/{grade}/not-enrolled")]
pub async fn get_not_enrolled(
    state: actix_web::web::Data<EnrollmentsState>,
    req: HttpRequest,
    available_subject_req: Path<AvailableSubjectReq>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!(
                "User {} getting new subjects not enrolled into.",
                claims.sub
            );
            match state
                .repo
                .db_get_available_subjects(
                    available_subject_req.student_id.as_str(),
                    available_subject_req.grade,
                )
                .await
            {
                Ok(non_enrollments) => Ok(HttpResponse::Ok().json(Json(non_enrollments))),
                Err(e) => Ok(HttpResponse::NotFound().json(e.to_string())),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

#[get("/{id}")]
pub async fn get_enrollment(
    state: actix_web::web::Data<EnrollmentsState>,
    req: HttpRequest,
    id: Path<String>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!(
                "User {} getting Enrollments with id {}",
                claims.sub,
                &id.clone()
            );
            match state.repo.db_get_enrollment(id.into_inner().as_str()).await {
                Ok(enrollment) => match enrollment {
                    Some(enrollment) => Ok(HttpResponse::Ok().json(enrollment)),
                    None => Ok(HttpResponse::NotFound().json("Enrollment not found")),
                },
                Err(e) => Ok(HttpResponse::NotFound().json(e.to_string())),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

#[get("/{subject_id}/{student_id}")]
pub async fn get_enrollment_for_subject_student(
    state: actix_web::web::Data<EnrollmentsState>,
    req: HttpRequest,
    enroll_path: Path<EnrollmentNew>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} getting Enrollment", claims.sub);

            match state
                .repo
                .db_get_enrollment_with_subject_student(
                    enroll_path.subject_id.as_str(),
                    enroll_path.student_id.as_str(),
                )
                .await
            {
                Ok(enrollments) => Ok(HttpResponse::Ok().json(Json(enrollments))),
                Err(e) => Ok(HttpResponse::NotFound().json(e.to_string())),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}
#[post("")]
pub async fn create_enrollment(
    state: actix_web::web::Data<EnrollmentsState>,
    req: HttpRequest,
    payload: Json<EnrollmentNew>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} Creating Enrollment", claims.sub);

            match state.repo.db_create_enrollment(payload.into_inner()).await {
                Ok(enrollment) => Ok(HttpResponse::Ok().json(Json(enrollment))),
                Err(e) => Ok(HttpResponse::NotFound().json(e.to_string())),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}
