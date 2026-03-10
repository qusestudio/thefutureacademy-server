use crate::domains::allocations::models::allocation::{Allocation, AllocationNew, TeachingAllocation};

#[async_trait::async_trait]
pub trait AllocationRepository {
    async fn db_set_allocation(&self, new_allocation: AllocationNew) -> sqlx::Result<Option<Allocation>>;
    async fn db_get_all_allocations(&self) -> sqlx::Result<Vec<Allocation>>;
    async fn db_get_instructor_allocations(&self, instructor_id: String) -> sqlx::Result<Vec<TeachingAllocation>>;
}