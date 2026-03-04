pub mod configuration;
pub mod domains;
pub mod infrastructure;

use crate::configuration::server::run;
use crate::configuration::state::build_state;
use dotenv::dotenv;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv().ok();

    let state = build_state().await;

    run(state).await
}
