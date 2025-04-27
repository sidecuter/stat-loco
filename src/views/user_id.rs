use crate::models::_entities::user_ids;
use loco_rs::{
    controller::views::pagination::{Pager, PagerMeta},
    model::query::{PageResponse, PaginationQuery},
};
use sea_orm::DerivePartialModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(DerivePartialModel, Serialize, Deserialize)]
#[sea_orm(entity = "user_ids::Entity", from_query_result)]
pub struct UserId {
    user_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListResponse {
    pub user_id: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaginationResponse {}

impl ListResponse {
    #[must_use]
    pub fn new(user: &user_ids::Model) -> Self {
        Self {
            user_id: user.user_id.to_string(),
            created_at: user.created_at.to_string(),
        }
    }
}

impl From<user_ids::Model> for ListResponse {
    fn from(user: user_ids::Model) -> Self {
        Self::new(&user)
    }
}

impl PaginationResponse {
    #[must_use]
    pub fn response(
        data: PageResponse<user_ids::Model>,
        pagination_query: &PaginationQuery,
    ) -> Pager<Vec<ListResponse>> {
        Pager {
            results: data
                .page
                .into_iter()
                .map(ListResponse::from)
                .collect::<Vec<ListResponse>>(),
            info: PagerMeta {
                page: pagination_query.page,
                page_size: pagination_query.page_size,
                total_pages: data.total_pages,
                total_items: data.total_items,
            },
        }
    }
}
