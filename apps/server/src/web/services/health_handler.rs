use crate::web::HEALTH_BODY;

pub async fn health_handler() -> &'static str {
    HEALTH_BODY
}
