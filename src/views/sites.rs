use crate::models::_entities::sites;
use crate::views::user_id::UserId;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::DerivePartialModel;
use serde::{Deserialize, Serialize};

#[derive(DerivePartialModel, Serialize, Deserialize)]
#[sea_orm(entity = "sites::Entity", from_query_result)]
pub struct Site {
    #[serde(flatten)]
    #[sea_orm(nested)]
    user_id: Option<UserId>,
    endpoint: Option<String>,
    created_at: DateTimeWithTimeZone,
}
