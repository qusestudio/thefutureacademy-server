use sqlx::{Error, PgPool};
use crate::domains::billing::checkouts::models::checkout::{Checkout, CheckoutFromDB, CheckoutStatus};
use crate::domains::billing::checkouts::repo::checkout_repository::CheckoutRepository;

pub struct PostgresCheckoutRepository {
    pub pg_pool: PgPool,
}

#[async_trait::async_trait]
impl CheckoutRepository for PostgresCheckoutRepository {
    async fn get_checkout(&self, id: &str) -> sqlx::Result<Checkout, Error> {
        let checkout_from_db: Option<CheckoutFromDB> = sqlx::query_as("select * from checkouts WHERE id=$1")
            .bind(id)
            .fetch_optional(&self.pg_pool)
            .await?;

        match checkout_from_db {
            Some(db_ckt) => Ok(Checkout::from_checkout_db(db_ckt)),
            None => Err(Error::RowNotFound),
        }
    }

    async fn get_checkout_by_student_id(&self, student_id: &str) -> sqlx::Result<Checkout, Error> {
        let checkout_from_db: Option<CheckoutFromDB> = sqlx::query_as("SELECT * FROM checkouts WHERE student_id=$1")
            .bind(student_id)
            .fetch_optional(&self.pg_pool)
            .await?;

        match checkout_from_db {
            Some(db_ckt) => Ok(Checkout::from_checkout_db(db_ckt)),
            None => Err(Error::RowNotFound),
        }
    }

    async fn create_checkout(&self, checkout: Checkout) -> sqlx::Result<Checkout, Error> {
        let checkout_from_db: Option<CheckoutFromDB> = sqlx::query_as("
        INSERT INTO checkouts (id, student_id, plan_id, status, gateway_reference)
        VALUES ($1, $2, $3, $4, $5) RETURNING *")
            .bind(&checkout.id)
            .bind(&checkout.student_id)
            .bind(&checkout.plan_id)
            .bind(&checkout.status.to_string())
            .bind(&checkout.gateway_reference)
            .fetch_optional(&self.pg_pool)
            .await?;

        match checkout_from_db {
            Some(db_ckt) => Ok(Checkout::from_checkout_db(db_ckt)),
            None => Err(Error::RowNotFound),
        }
    }

    async fn update_checkout_status(&self, status: CheckoutStatus, checkout_id: &str) -> sqlx::Result<bool, Error> {
        log::info!("Updating checkout status for checkout_id: {} to {}", checkout_id, status.clone().to_string());
        let _update_status = sqlx::query(" UPDATE checkouts SET status = $1 WHERE id = $2")
            .bind(status.to_string())
            .bind(checkout_id)
            .execute(&self.pg_pool)
            .await?;

        Ok(true)
    }
}