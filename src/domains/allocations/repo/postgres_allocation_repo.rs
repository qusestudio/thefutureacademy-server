use crate::domains::allocations::models::allocation::{Allocation, AllocationNew, TeachingAllocation};
use crate::domains::allocations::repo::allocation_repo::AllocationRepository;

pub struct PostgresAllocationRepo {
    pub pg_pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl AllocationRepository for PostgresAllocationRepo {
    async fn db_set_allocation(&self, new_allocation: AllocationNew) -> sqlx::Result<Option<Allocation>> {
        let allocation = Allocation::new(new_allocation);
        let allocation = sqlx::query_as("INSERT INTO teaching_allocations VALUES ($1, $2, $3, $4) RETURNING *")
            .bind(&allocation.id)
            .bind(&allocation.subject_id)
            .bind(&allocation.instructor_id)
            .bind(&allocation.created_at)
            .fetch_optional(&self.pg_pool)
            .await?;

        Ok(allocation)
    }

    async fn db_get_allocations(&self, instructor_id: String) -> sqlx::Result<Vec<TeachingAllocation>> {
        let allocations = sqlx::query_as(
            "SELECT
                i.cognito_id AS instructor_id,
                i.name AS instructor_name,
                sub.id AS subject_id,
                sub.title AS subject_title,
                sub.grade,
                sub.term
            FROM
                instructors i
                    INNER JOIN
                enrollments e ON i.cognito_id = e.student_id
                    INNER JOIN
                subjects sub ON e.subject_id = sub.id
            WHERE
                i.cognito_id=$1;
            ",
        )
            .bind(instructor_id)
            .fetch_all(&self.pg_pool)
            .await?;
        Ok(allocations)
    }
}