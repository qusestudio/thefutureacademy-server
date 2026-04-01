use crate::configuration::state::AppState;
use crate::domains::allocations::models::allocation::AllocationNew;
use crate::infrastructure::middleware::middleware::middleware;
use actix_web::web::{Json, Path};
use actix_web::{HttpRequest, HttpResponse, web, post, get, delete};
use serde_json::json;

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
                    match state.allocations.allocate_instructor(payload.into_inner()).await
                    {
                        Ok(allocation) =>  {
                            Ok(HttpResponse::Ok().json(allocation))
                        },
                        Err(error) => {
                            log::error!("Error setting allocation: {}", error);
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

#[get("")]
pub async fn get_all_allocations(
    req: HttpRequest,
    state: web::Data<AppState>
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            match claims.custom_role.as_str() {
                "admin" => {
                    log::info!("Admin user {} getting allocations.", claims.sub);
                    match state.allocations.get_all_allocations().await {
                        Ok(allocations) => Ok(HttpResponse::Ok().json(allocations)),
                        Err(error) => {
                            Ok(HttpResponse::InternalServerError().json(error.to_string()))
                        }
                    }
                },
                _ => Ok(HttpResponse::Forbidden().body("Access denied.")),
            }
        },
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
            log::info!("User {} getting instructor allocations for {}", claims.sub, &instructor_id);
            match claims.custom_role.as_str() {
                "admin" | "instructor" => {
                    match state.allocations.get_instructor_allocations(instructor_id.into_inner().as_str())
                        .await
                    {
                        Ok(allocations) => {
                            log::info!("No. of results: {}", allocations.len());
                            Ok(HttpResponse::Ok().json(allocations))
                        },
                        Err(error) => Ok(HttpResponse::InternalServerError().json(error.to_string())),
                    }
                }
                _ => Ok(HttpResponse::Forbidden().body("Access denied.")),
            }
        }
        Err(err) => Ok(HttpResponse::Unauthorized().json(format!("{}", err))),
    }
}

#[delete("")]
pub async fn delete_allocations(
    req: HttpRequest,
    state: web::Data<AppState>,
    payload: Json<Vec<String>>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => match claims.custom_role.as_str() {
            "admin" => {
                log::info!("Admin deleting allocations"); 
                match state
                    .allocations
                    .repo
                    .db_delete_allocations(payload.into_inner())
                    .await
                {
                    Ok(rows_affected) => Ok(HttpResponse::Ok().json(json!({
                        "rows_affected": rows_affected
                    }))),
                    Err(e) => {
                        log::error!("Error deleting allocations: {}", e);
                        Ok(HttpResponse::InternalServerError().json(e.to_string()))
                    },
                }
            }
            _ => Ok(HttpResponse::Forbidden().json("Not allowed")),
        },
        Err(e) => Ok(HttpResponse::Unauthorized().json(format!("{}", e))),
    }
}
