use crate::data::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDmRequest {
    pub target_user_id: UserId,
}
