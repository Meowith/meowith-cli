use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppInfo {
    pub bucket_id: Uuid,
    pub app_id: Uuid,
    pub node: String,
}
