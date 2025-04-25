use crate::models::_entities::user_ids;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewResponse {
    pub id: String,
    pub user_id: String,
    pub created_at: String,
}

impl NewResponse {
    #[must_use]
    pub fn new(user: &user_ids::Model) -> Self {
        Self {
            id: user.id.to_string(),
            user_id: user.user_id.to_string(),
            created_at: user.created_at.to_string(),
        }
    }
}
