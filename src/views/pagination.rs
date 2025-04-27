use loco_rs::controller::views::pagination::{Pager, PagerMeta};
use loco_rs::model::query::{PageResponse, PaginationQuery};
use sea_orm::{DatabaseConnection, PaginatorTrait, PartialModelTrait, SelectModel, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PaginationResponse {}

impl PaginationResponse {
    #[must_use]
    pub fn response<T: PartialModelTrait>(
        data: PageResponse<T>,
        pagination_query: &PaginationQuery,
    ) -> Pager<Vec<T>> {
        Pager {
            results: data.page,
            info: PagerMeta {
                page: pagination_query.page,
                page_size: pagination_query.page_size,
                total_pages: data.total_pages,
                total_items: data.total_items,
            },
        }
    }

    /// Paginated view on selected data.
    ///
    /// # Errors
    /// - Returns an error if database query fails
    pub async fn paginate<'db, M>(
        db: &DatabaseConnection,
        entity: Selector<SelectModel<M>>,
        pagination_query: &PaginationQuery,
    ) -> loco_rs::Result<PageResponse<M>>
    where
        M: PartialModelTrait + Send + Sync + 'db,
        SelectModel<M>: Send,
    {
        let page = if pagination_query.page <= 1 {
            0
        } else {
            pagination_query.page - 1
        };

        let query = entity.paginate(db, pagination_query.page_size);
        let total_pages_and_items = query.num_items_and_pages().await?;
        let page: Vec<M> = query.fetch_page(page).await?;

        let paginated_response = PageResponse {
            page,
            total_pages: total_pages_and_items.number_of_pages,
            total_items: total_pages_and_items.number_of_items,
        };

        Ok(paginated_response)
    }
}
