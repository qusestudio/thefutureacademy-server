use crate::domains::users::admin::models::admin::{Admin, AdminNew};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait AdminRepository {
    async fn db_get_admin_by_cognito(&self, cognito_id: &String) -> sqlx::Result<Admin, sqlx::Error>;
    async fn db_create_admin(&self, admin_new: AdminNew) -> sqlx::Result<Admin, sqlx::Error>;
}