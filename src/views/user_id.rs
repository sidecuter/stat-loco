use crate::models::_entities::user_ids;
use sea_orm::DerivePartialModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewResponse {
    pub user_id: String,
    pub created_at: String,
}

impl NewResponse {
    #[must_use]
    pub fn new(user: &user_ids::Model) -> Self {
        Self {
            user_id: user.user_id.to_string(),
            created_at: user.created_at.to_string(),
        }
    }
}

#[derive(DerivePartialModel, Serialize, Deserialize)]
#[sea_orm(entity = "user_ids::Entity", from_query_result)]
pub struct UserId {
    user_id: Uuid,
}
