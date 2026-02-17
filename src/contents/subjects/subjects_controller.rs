use crate::authentication::middleware::middleware::middleware;
use crate::contents::subjects::models::subject::SubjectNew;
use crate::contents::subjects::subjects_state::SubjectsState;
use actix_web::web::{Json, Path};
use actix_web::{HttpRequest, HttpResponse, get, post, web};
use serde::{Deserialize};

#[get("/{id}")]
pub async fn get_subject(
    req: HttpRequest,
    state: web::Data<SubjectsState>,
    id: Path<String>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            let user_id = claims.sub;
            log::info!(
                "User {} requesting subject with id {}",
                &user_id,
                &id.clone()
            );
            match state.repo.db_get_subject(&id.clone()).await {
                Ok(subject) => Ok(HttpResponse::Ok().json(Json(subject))),
                Err(_) => Ok(HttpResponse::NotFound().json("Subject not found")),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

#[get("/{instructor_id}/subjects")]
pub async fn get_subjects_by_instructor(
    req: HttpRequest,
    state: web::Data<SubjectsState>,
    instructor_id: Path<String>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            let user_id = claims.sub;
            log::info!(
                "User {} requesting subjects with instructor Id {}",
                &user_id,
                &instructor_id.clone()
            );
            match state
                .repo
                .db_get_subjects_by_instructor(&instructor_id.clone())
                .await
            {
                Ok(subjects) => Ok(HttpResponse::Ok().json(Json(subjects))),
                Err(_) => Ok(HttpResponse::NotFound().json("Subjects not found")),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

#[get("/{grade}/subjects")]
pub async fn get_subjects_by_grade(
    req: HttpRequest,
    state: web::Data<SubjectsState>,
    grade: Path<i32>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            let user_id = claims.sub;
            log::info!("User {} requesting subjects from grade {}", &user_id, grade);
            match state
                .repo
                .db_get_subjects_by_grade(grade.into_inner())
                .await
            {
                Ok(subjects) => Ok(HttpResponse::Ok().json(Json(subjects))),
                Err(_) => Ok(HttpResponse::NotFound().json("Subjects not found")),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

#[get("/{term}/subjects")]
pub async fn get_subjects_by_term(
    req: HttpRequest,
    state: web::Data<SubjectsState>,
    term: Path<i32>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            let user_id = claims.sub;
            log::info!("User {} requesting subjects from term {}", &user_id, term);
            match state.repo.db_get_subjects_by_term(term.into_inner()).await {
                Ok(subjects) => Ok(HttpResponse::Ok().json(Json(subjects))),
                Err(_) => Ok(HttpResponse::NotFound().json("Subjects not found")),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

#[derive(Deserialize)]
pub struct GradeTerm {
    pub grade: i32,
    pub term: i32,
}

#[get("/{grade}/terms/{term}/subjects")]
pub async fn get_subjects_by_term_and_grade(
    req: HttpRequest,
    state: web::Data<SubjectsState>,
    path: Path<GradeTerm>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            let grade = path.grade;
            let term = path.term;
            let user_id = claims.sub;
            log::info!(
                "User {} requesting subjects from grade {}, term {}",
                &user_id,
                grade,
                term
            );
            match state
                .repo
                .db_get_subjects_by_term_and_grade(term, grade)
                .await
            {
                Ok(subjects) => Ok(HttpResponse::Ok().json(Json(subjects))),
                Err(_) => Ok(HttpResponse::NotFound().json("Subjects not found")),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

#[post("")]
pub async fn create_subject(
    req: HttpRequest,
    state: web::Data<SubjectsState>,
    payload: Json<SubjectNew>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            let user_id = claims.sub;
            log::info!("User {} creating a new subject", &user_id);
            match state.repo.db_create_subject(&payload.into_inner()).await {
                Ok(subjects) => Ok(HttpResponse::Ok().json(Json(subjects))),
                Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}
