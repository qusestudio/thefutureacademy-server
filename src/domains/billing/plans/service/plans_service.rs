use std::io::Error;
use std::sync::Arc;
use actix_web::web::Data;
use crate::domains::billing::plans::models::plan::{Plan, PlanNew};
use crate::domains::billing::plans::repo::plan_repo::PlanRepository;
use crate::infrastructure::event_bus::event_bus::EventBus;

pub struct PlansService {
    pub repo: Arc<dyn PlanRepository + Sync + Send>,
    pub event_bus: Data<EventBus>,
}

impl PlansService {
    fn unwrap_repo_result<T>(
        result: Result<Option<T>, impl std::fmt::Display>,
        error_msg: &str,
    ) -> Result<T, Error> {
        match result {
            Ok(Some(value)) => Ok(value),
            Ok(None) => Err(Error::other(error_msg)),
            Err(error) => {
                log::error!("{}: {}", error_msg, error);
                Err(Error::other(error.to_string()))
            }
        }
    }
    pub async fn create_plan(&self, np: PlanNew) -> Result<Plan, Error> {
        let plan = Plan::new(&np);
        Self::unwrap_repo_result(self.repo.create_plan(plan).await, "Error creating plan")
    }

    pub async fn get_plan_by_id(&self, id: String) -> Result<Plan, Error> {
        Self::unwrap_repo_result(self.repo.get_plan_by_id(id).await, "Error getting plan by id")
    }
    
    pub async fn get_plan_by_grade(&self, grade: i32) -> Result<Plan, Error> {
        Self::unwrap_repo_result(self.repo.get_plan_by_grade(grade).await, "Error getting plan by grade")
    }

    pub async fn get_all_plans(&self) -> Result<Vec<Plan>, Error> {
        match self.repo.get_all_plans().await {
            Ok(plans) => Ok(plans),
            Err(error) => {
                log::error!("Error getting all plans: {}", error);
                Err(Error::other(error))
            }
        }
    }
}