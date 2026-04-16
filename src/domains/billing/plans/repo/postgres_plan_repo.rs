use crate::domains::billing::plans::models::plan::Plan;
use crate::domains::billing::plans::repo::plan_repo::PlanRepository;
use sqlx::Error;

pub struct PostgresPlanRepo {
    pub pg_pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl PlanRepository for PostgresPlanRepo {
    async fn create_plan(&self, plan: Plan) -> sqlx::Result<Option<Plan>, Error> {
        let plan = sqlx::query_as(
            "INSERT INTO plans (id, name, price, duration, is_active, grade)
                VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        )
        .bind(&plan.id)
        .bind(&plan.name)
        .bind(&plan.price)
        .bind(&plan.duration)
        .bind(&plan.is_active)
        .bind(&plan.grade)
        .fetch_optional(&self.pg_pool)
        .await?;

        Ok(plan)
    }
    async fn get_plan_by_id(&self, id: String) -> sqlx::Result<Option<Plan>, Error> {
        let plan = sqlx::query_as("SELECT * FROM plans WHERE id = $1")
            .bind(&id)
            .fetch_optional(&self.pg_pool)
            .await?;

        Ok(plan)
    }
    async fn get_plan_by_grade(&self, grade: i32) -> sqlx::Result<Option<Plan>, Error> {
        let plan = sqlx::query_as("SELECT * FROM plans WHERE grade = $1")
            .bind(&grade)
            .fetch_optional(&self.pg_pool)
            .await?;
        
        Ok(plan)
    }
    async fn get_all_plans(&self) -> sqlx::Result<Vec<Plan>, Error> {
        let plans = sqlx::query_as("SELECT * FROM plans")
            .fetch_all(&self.pg_pool)
            .await?;

        Ok(plans)
    }
}

#[cfg(test)]
mod tests {
    use crate::domains::billing::plans::models::plan::Plan;
    use crate::domains::billing::plans::repo::plan_repo::PlanRepository;
    use crate::domains::billing::plans::repo::postgres_plan_repo::PostgresPlanRepo;
    use uuid::Uuid;

    fn make_test_plan() -> Plan {
        Plan {
            id: Uuid::now_v7().to_string(),
            name: "Test Plan".to_string(),
            price: 1000,
            duration: 30,
            is_active: true,
            grade: 0,
        }
    }
    // ── create_plan ──────────────────────────────────────────────────────────────────────────────

    #[sqlx::test]
    async fn create_plan_returns_created_plan(pool: sqlx::PgPool) {
        let repo = PostgresPlanRepo { pg_pool: pool };
        let plan = make_test_plan();

        let result = repo.create_plan(plan.clone()).await;

        assert!(result.is_ok());
        let returned = result.unwrap();
        assert!(returned.is_some());
        let returned = returned.unwrap();
        assert_eq!(returned.id, plan.id);
        assert_eq!(returned.name, plan.name);
        assert_eq!(returned.price, plan.price);
        assert_eq!(returned.duration, plan.duration);
        assert_eq!(returned.is_active, plan.is_active);
    }

    #[sqlx::test]
    async fn create_plan_duplicate_id_returns_error(pool: sqlx::PgPool) {
        let repo = PostgresPlanRepo { pg_pool: pool };
        let plan = make_test_plan();

        repo.create_plan(plan.clone()).await.unwrap();
        let result = repo.create_plan(plan).await; // same ID → PK violation

        assert!(result.is_err());
    }

    // ── get_plan_by_id ───────────────────────────────────────────────────────────────────────────
    #[sqlx::test]
    async fn get_plan_by_id_returns_correct_plan(pool: sqlx::PgPool) {
        let repo = PostgresPlanRepo { pg_pool: pool };
        let plan = make_test_plan();
        repo.create_plan(plan.clone()).await.unwrap();

        let result = repo.get_plan_by_id(plan.id.clone()).await;

        assert!(result.is_ok());
        let fetched = result.unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().id, plan.id);
    }

    #[sqlx::test]
    async fn get_plan_by_id_returns_none_for_missing_id(pool: sqlx::PgPool) {
        let repo = PostgresPlanRepo { pg_pool: pool };

        let result = repo.get_plan_by_id(Uuid::now_v7().to_string()).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    // ── get_all_plans ────────────────────────────────────────────────────────────────────────────
    #[sqlx::test]
    async fn get_all_plans_returns_empty_when_no_plans(pool: sqlx::PgPool) {
        let repo = PostgresPlanRepo { pg_pool: pool };

        let result = repo.get_all_plans().await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[sqlx::test]
    async fn get_all_plans_returns_all_inserted_plans(pool: sqlx::PgPool) {
        let repo = PostgresPlanRepo { pg_pool: pool };
        let plans = vec![make_test_plan(), make_test_plan(), make_test_plan()];

        for plan in &plans {
            repo.create_plan(plan.clone()).await.unwrap();
        }

        let result = repo.get_all_plans().await;

        assert!(result.is_ok());
        let fetched = result.unwrap();
        assert_eq!(fetched.len(), plans.len());

        // Verify every inserted ID is present in the result
        for plan in &plans {
            assert!(fetched.iter().any(|p| p.id == plan.id));
        }
    }

    #[sqlx::test]
    async fn get_all_plans_returns_correct_field_values(pool: sqlx::PgPool) {
        let repo = PostgresPlanRepo { pg_pool: pool };
        let plan = make_test_plan();
        repo.create_plan(plan.clone()).await.unwrap();

        let fetched = repo.get_all_plans().await.unwrap();
        let found = fetched.iter().find(|p| p.id == plan.id).unwrap();

        assert_eq!(found.name, plan.name);
        assert_eq!(found.price, plan.price);
        assert_eq!(found.duration, plan.duration);
        assert_eq!(found.is_active, plan.is_active);
    }
}
