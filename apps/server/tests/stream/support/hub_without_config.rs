use app::hub::Hub;
use std::sync::Arc;

pub fn hub_without_config() -> Arc<Hub> {
    Arc::new(Hub::new())
}
