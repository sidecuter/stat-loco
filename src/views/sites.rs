use sea_orm::{DerivePartialModel};
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::_entities::user_ids;
use crate::models::_entities::sites;

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
    user_id: Uuid
}
