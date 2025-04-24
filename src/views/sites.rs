use loco_rs::controller::views::pagination::{Pager, PagerMeta};
use loco_rs::prelude::query::{PaginationQuery, PageResponse};
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

#[derive(Debug, Deserialize, Serialize)]
pub struct PaginationResponse {}

impl PaginationResponse {
    #[must_use]
    pub async fn response(
        data: PageResponse<Site>,
        pagination_query: &PaginationQuery
    ) -> loco_rs::Result<Pager<Vec<Site>>> {
        Ok(Pager {
            results: data.page,
            info: PagerMeta {
                page: pagination_query.page,
                page_size: pagination_query.page_size,
                total_pages: data.total_pages,
                total_items: data.total_items
            }
        })
    }
}
