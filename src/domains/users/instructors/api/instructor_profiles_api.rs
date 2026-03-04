use actix_web::{post, get, put, HttpRequest, HttpResponse, delete};
use actix_web::web::{Data, Json, Path};
use crate::configuration::state::AppState;
use crate::infrastructure::middleware::middleware::middleware;
use crate::domains::users::instructors::models::instructor_profile::InstructorProfileNew;

#[post("")]
pub async fn create_instructor_profile(
    state: Data<AppState>,
    req: HttpRequest,
    np: Json<InstructorProfileNew>
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} creating instructor profile", claims.sub);
            match state
                .instructor_profiles
                .repo
                .create_instructor_profile(np.into_inner())
                .await {
                Ok(profile) => {
                    match profile {
                        Some(profile) => {
                            Ok(HttpResponse::Ok().json(profile))
                        },
                        None => {
                            Ok(HttpResponse::InternalServerError().json("Either profile could not be created or something went wrong"))
                        }
                    }
                },
                Err(e) => {
                    Ok(HttpResponse::NotFound().json(e.to_string()))
                }
            }
        }
        Err(error) => {
            log::error!("{:?}", error);
            Ok(HttpResponse::Unauthorized().json(error.to_string()))
        }
    }
}

#[get("/{instructor_id}/profile")]
pub async fn get_instructor_profile(
    state: Data<AppState>,
    req: HttpRequest,
    instructor_id: Path<String>
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} getting instructor profile", claims.sub);
            match state
                .instructor_profiles
                .repo
                .get_instructor_profile(instructor_id.into_inner())
                .await {
                Ok(profile) => {
                    match profile {
                        Some(profile) => {
                            Ok(HttpResponse::Ok().json(profile))
                        },
                        None => {
                            Ok(HttpResponse::NotFound().json("Instructor profile not found"))
                        }
                    }
                },
                Err(e) => {
                    Ok(HttpResponse::NotFound().json(e.to_string()))
                }
            }
        }
        Err(error) => {
            log::error!("{:?}", error);
            Ok(HttpResponse::Unauthorized().json(error.to_string()))
        }
    }
}

#[put("/{instructor_id}")]
pub async fn update_instructor_profile(
    state: Data<AppState>,
    req: HttpRequest,
    id: Path<String>,
    np: Json<InstructorProfileNew>
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} updating instructor profile", claims.sub);
            match state
                .instructor_profiles
                .repo
                .update_instructor_profile(id.into_inner(), np.into_inner())
                .await {
                Ok(is_updated) => {
                    match is_updated {
                        true => {
                            Ok(HttpResponse::Ok().json("Instructor profile updated"))
                        },
                        false => {
                            Ok(HttpResponse::NotFound().json("Instructor profile could not be updated"))
                        }
                    }
                },
                Err(e) => {
                    Ok(HttpResponse::InternalServerError().json(e.to_string()))
                }
            }
        }
        Err(error) => {
            log::error!("{:?}", error);
            Ok(HttpResponse::Unauthorized().json(error.to_string()))
        }
    }
}

#[delete("/{instructor_id}")]
pub async fn delete_instructor_profile(
    state: Data<AppState>,
    req: HttpRequest,
    instructor_id: Path<String>
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} delete instructor profile", claims.sub);
            match state
                .instructor_profiles
                .repo
                .delete_instructor_profile(instructor_id.into_inner())
                .await {
                Ok(deleted) => {
                    match deleted {
                        true => {
                            Ok(HttpResponse::Ok().json("Instructor profile deleted. It might not exist."))
                        },
                        false => {
                            Ok(HttpResponse::NotFound().json("Instructor profile could not be deleted. It might not exist."))
                        }
                    }
                },
                Err(e) => {
                    Ok(HttpResponse::InternalServerError().json(e.to_string()))
                }
            }
        }
        Err(error) => {
            log::error!("{:?}", error);
            Ok(HttpResponse::Unauthorized().json(error.to_string()))
        }
    }
}