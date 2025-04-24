use loco_rs::model::query::{PageResponse, PaginationQuery};
use sea_orm::{DatabaseConnection, PaginatorTrait, PartialModelTrait, SelectModel, Selector};

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
