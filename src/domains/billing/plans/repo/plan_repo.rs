use sqlx::Error;
use crate::domains::billing::plans::models::plan::{Plan};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait PlanRepository {
    async fn create_plan(&self, plan: Plan) -> sqlx::Result<Option<Plan>, Error>;
    async fn get_plan_by_id(&self, id: String) -> sqlx::Result<Option<Plan>, Error>;
    async fn get_plan_by_grade(&self, grade: i32) -> sqlx::Result<Option<Plan>, Error>;
    async fn get_all_plans(&self) -> sqlx::Result<Vec<Plan>, sqlx::Error>;
}