use crate::configuration::state::AppState;
use crate::domains::allocations::models::allocation::AllocationNew;
use crate::infrastructure::middleware::middleware::middleware;
use actix_web::web::Path;
use actix_web::{HttpRequest, HttpResponse, web, post, get};

#[post("")]
pub async fn set_teaching_allocation(
    req: HttpRequest,
    state: web::Data<AppState>,
    payload: web::Json<AllocationNew>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            // the admin person is the one setting this allocation
            match claims.custom_role.as_str() {
                "admin" => {
                    match state
                        .allocations
                        .repo
                        .db_set_allocation(payload.into_inner())
                        .await
                    {
                        Ok(allocation) => match allocation {
                            Some(allocation) => Ok(HttpResponse::Ok().json(allocation)),
                            None => Ok(HttpResponse::NotFound().finish()),
                        },
                        Err(error) => {
                            Ok(HttpResponse::InternalServerError().json(error.to_string()))
                        }
                    }
                }
                _ => Ok(HttpResponse::Forbidden().body("Not allowed")),
            }
        }
        Err(err) => Ok(HttpResponse::Unauthorized().json(format!("{}", err))),
    }
}

#[get("/{instructor_id}/allocations")]
pub async fn get_instructor_allocations(
    req: HttpRequest,
    state: web::Data<AppState>,
    instructor_id: Path<String>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("Admin {} getting instructor allocations for {}", claims.sub, &instructor_id);
            match state
                .allocations
                .repo
                .db_get_allocations(instructor_id.into_inner())
                .await
            {
                Ok(allocations) => Ok(HttpResponse::Ok().json(allocations)),
                Err(error) => Ok(HttpResponse::InternalServerError().json(error.to_string())),
            }
        }
        Err(err) => Ok(HttpResponse::Unauthorized().json(format!("{}", err))),
    }
}
