use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiError {
    #[serde(rename = "statusCode")]
    pub status_code: u16,
    pub message: String,
}
