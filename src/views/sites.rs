use crate::models::_entities::sites;
use crate::models::_entities::user_ids;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::DerivePartialModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(DerivePartialModel, Serialize, Deserialize)]
#[sea_orm(entity = "sites::Entity", from_query_result)]
pub struct Site {
    #[serde(flatten)]
    #[sea_orm(nested)]
    user_id: Option<UserId>,
    endpoint: Option<String>,
    created_at: DateTimeWithTimeZone,
}

#[derive(DerivePartialModel, Serialize, Deserialize)]
#[sea_orm(entity = "user_ids::Entity", from_query_result)]
pub struct UserId {
    user_id: Uuid,
}
