use loco_rs::controller::views::pagination::{Pager, PagerMeta};
use loco_rs::model::query::{PageResponse, PaginationQuery};
use sea_orm::PartialModelTrait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PaginationResponse {}

impl PaginationResponse {
    #[must_use]
    pub async fn response<T: PartialModelTrait>(
        data: PageResponse<T>,
        pagination_query: &PaginationQuery
    ) -> loco_rs::Result<Pager<Vec<T>>> {
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