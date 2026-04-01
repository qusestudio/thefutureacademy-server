#[derive(Clone)]
pub struct EnvironmentVars {
    pub port: u16,
    pub host: String,
    pub frontend_origin: String,
    pub studio: String,
    pub space: String,
    pub instructors: String,
    pub yoco_webhook_origin: String,
}
// todo: Implement the factory pattern so that only one object is used.
impl EnvironmentVars {
    pub fn init() -> Self {
        let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string())
            .parse::<u16>()
            .unwrap();
        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let frontend_origin =
            std::env::var("FRONTEND_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".to_string());
        let studio =
            std::env::var("STUDIO").unwrap_or_else(|_| "http://localhost:3000".to_string());
        let yoco_webhook_origin = std::env::var("PAYMENT_WEBHOOKS_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".to_string());
        let space =
            std::env::var("SPACE").unwrap_or_else(|_| "http://localhost:3000".to_string());
        let instructors =
            std::env::var("INSTRUCTORS").unwrap_or_else(|_| "http://localhost:3000".to_string());

        Self {
            port,
            host,
            frontend_origin,
            yoco_webhook_origin,
            studio,
            space,
            instructors
        }
    }
}