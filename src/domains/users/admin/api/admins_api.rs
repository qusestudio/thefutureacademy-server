use crate::configuration::state::AppState;
use crate::domains::users::admin::models::admin::{Admin, AdminNew};
use crate::domains::users::students::models::student::{Student, StudentNew};
use crate::domains::users::students::service::students_service::StudentsService;
use crate::infrastructure::middleware::middleware::middleware;
use actix_web::web::{Json, Path};
use actix_web::{HttpRequest, HttpResponse, get, post, web};

#[get("/{cognito_id}")]
pub async fn get_admin_by_cognito(
    req: HttpRequest,
    state: web::Data<AppState>,
    cognito_id: Path<String>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("Getting admin with id {}", &cognito_id.clone());
            match claims.custom_role.as_str() {
                "admin" => {
                    match state
                        .admins
                        .repo
                        .db_get_admin_by_cognito(&cognito_id.clone())
                        .await
                    {
                        Ok(admin) => Ok(HttpResponse::Ok().json(Json(admin))),
                        Err(_) => Ok(HttpResponse::NotFound().json("Admin not found")),
                    }
                }
                _ => Ok(HttpResponse::Unauthorized().json("Access denied")),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

#[post("")]
pub async fn create_admin(
    req: HttpRequest,
    state: web::Data<AppState>,
    payload: Json<AdminNew>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("Creating admin with id {}", &claims.sub);
            match claims.custom_role.as_str() {
                "admin" => {
                    match state
                        .admins
                        .repo
                        .db_create_admin(payload.into_inner())
                        .await
                    {
                        Ok(admin) => Ok(HttpResponse::Ok().json(Json(admin))),
                        Err(e) => Ok(HttpResponse::NotFound().json(e.to_string())),
                    }
                }
                _ => Ok(HttpResponse::Unauthorized().json("Access denied")),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}
