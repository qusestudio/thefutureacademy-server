use crate::configuration::state::AppState;
use crate::domains::learning::subjects::models::subject::SubjectNew;
use crate::infrastructure::middleware::middleware::middleware;
use actix_web::web::{Json, Path};
use actix_web::{HttpRequest, HttpResponse, delete, get, post, web};
use serde::Deserialize;
use serde_json::json;

#[get("/{id}")]
pub async fn get_subject(
    req: HttpRequest,
    state: web::Data<AppState>,
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
            match state.subjects.repo.db_get_subject(&id.clone()).await {
                Ok(subject) => Ok(HttpResponse::Ok().json(Json(subject))),
                Err(_) => Ok(HttpResponse::NotFound().json("Subject not found")),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

#[get("")]
pub async fn get_all_subjects(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => match claims.custom_role.as_str() {
            "admin" => match state.subjects.repo.db_get_all_subjects().await {
                Ok(subjects) => Ok(HttpResponse::Ok().json(Json(subjects))),
                Err(e) => Ok(HttpResponse::InternalServerError().json(format!("{:?}", e))),
            },
            _ => Ok(HttpResponse::Forbidden().json("Not allowed")),
        },
        Err(e) => Ok(HttpResponse::Unauthorized().json(format!("{}", e))),
    }
}

#[get("/{grade}/subjects")]
pub async fn get_subjects_by_grade(
    req: HttpRequest,
    state: web::Data<AppState>,
    grade: Path<i32>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            let user_id = claims.sub;
            log::info!("User {} requesting subjects from grade {}", &user_id, grade);
            match state
                .subjects
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
    state: web::Data<AppState>,
    term: Path<i32>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            let user_id = claims.sub;
            log::info!("User {} requesting subjects from term {}", &user_id, term);
            match state
                .subjects
                .repo
                .db_get_subjects_by_term(term.into_inner())
                .await
            {
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
    state: web::Data<AppState>,
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
                .subjects
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
    state: web::Data<AppState>,
    payload: Json<SubjectNew>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            let user_id = claims.sub;
            log::info!("User {} creating a new subject", &user_id);
            match state
                .subjects
                .repo
                .db_create_subject(&payload.into_inner())
                .await
            {
                Ok(subjects) => Ok(HttpResponse::Ok().json(Json(subjects))),
                Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

#[delete("")]
pub async fn delete_subjects(
    req: HttpRequest,
    state: web::Data<AppState>,
    payload: Json<Vec<String>>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => match claims.custom_role.as_str() {
            "admin" => {
                log::info!("Admin deleting subjects");
                match state
                    .subjects
                    .repo
                    .db_delete_subjects(payload.into_inner())
                    .await
                {
                    Ok(rows_affected) => Ok(HttpResponse::Ok().json(json!({
                        "rows_affected": rows_affected
                    }))),
                    Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
                }
            }
            _ => Ok(HttpResponse::Forbidden().json("Not allowed")),
        },
        Err(e) => Ok(HttpResponse::Unauthorized().json(format!("{}", e))),
    }
}
