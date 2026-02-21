use sqlx::{Error, PgPool};
use crate::payments::models::checkout::{Checkout};
use crate::payments::repo::checkout_repository::CheckoutRepository;

pub struct PostgresCheckoutRepository {
    pub pg_pool: PgPool,
}

#[async_trait::async_trait]
impl CheckoutRepository for PostgresCheckoutRepository {
    async fn get_checkout(&self, id: &str) -> sqlx::Result<Option<Checkout>, Error> {
        let checkout = sqlx::query_as("select * from checkouts WHERE id=$1")
            .bind(id)
            .fetch_optional(&self.pg_pool)
            .await?;
        Ok(checkout)
    }

    async fn get_checkout_by_student_id(&self, student_id: &str) -> sqlx::Result<Option<Checkout>, Error> {
        let checkout = sqlx::query_as("SELECT * FROM checkouts WHERE student_id=$1")
            .bind(student_id)
            .fetch_optional(&self.pg_pool)
            .await?;
        
        Ok(checkout)
    }

    async fn create_checkout(&self, checkout: Checkout) -> sqlx::Result<Checkout, Error> {
        let checkout = sqlx::query_as("
        INSERT INTO checkouts (id, student_id, amount, status, month, year, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *")
            .bind(&checkout.id)
            .bind(&checkout.student_id)
            .bind(&checkout.amount)
            .bind(&checkout.status)
            .bind(&checkout.month)
            .bind(&checkout.year)
            .bind(&checkout.created_at)
            .bind(&checkout.updated_at)
            .fetch_one(&self.pg_pool)
            .await?;
        Ok(checkout)
    }

    async fn update_checkout_status(&self, status: &str, checkout_id: &str) -> sqlx::Result<bool, Error> {
        match self.get_checkout(checkout_id).await {
            Ok(checkout) => {
                match checkout {
                    Some(checkout) => {
                        // If checkout is present update its status
                        let _update_status = sqlx::query(" UPDATE checkouts SET status = $1 WHERE id = $2")
                            .bind(status)
                            .bind(checkout_id)
                            .execute(&self.pg_pool)
                            .await?;
                        Ok(true)
                    }
                    None => {
                        Err(Error::RowNotFound)
                    }
                }
            },
            Err(error) => {
                Err(error)
            },
        }
    }
}