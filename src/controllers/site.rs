#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::sites;
use crate::models::_entities::{user_ids, users};
use crate::models::sites::AddParams;
use crate::views::pagination::PaginationResponse;
use crate::views::sites::Site;
use axum::debug_handler;
use axum::extract::Query;
use loco_rs::model::query::PaginationQuery;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

const fn default_page() -> u64 {
    1
}
const fn default_size() -> u64 {
    50
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryWithFilter {
    user_id: Option<String>,
    #[serde(default = "default_size")]
    size: u64,
    #[serde(default = "default_page")]
    page: u64,
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<sites::Model> {
    let item = sites::Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Query(q): Query<QueryWithFilter>,
) -> Result<Response> {
    let _ = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let pagination_query = PaginationQuery {
        page_size: q.size,
        page: q.page,
    };
    let mut statement = sites::Entity::find().left_join(user_ids::Entity);
    statement = match q.user_id {
        Some(uid) => statement.filter(
            query::condition()
                .contains(user_ids::Column::UserId, uid)
                .build(),
        ),
        None => statement,
    };
    let paginated_sites = PaginationResponse::paginate::<Site>(
        &ctx.db,
        statement.into_partial_model::<Site>(),
        &pagination_query,
    )
    .await?;
    format::json(PaginationResponse::response::<Site>(
        paginated_sites,
        &pagination_query,
    ))
}

#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>,
    JsonValidateWithMessage(params): JsonValidateWithMessage<AddParams>,
) -> Result<Response> {
    let res = sites::ActiveModel::create_with_uuid(&ctx.db, &params).await;
    if let Err(err) = res {
        tracing::info!(
            message = err.to_string(),
            user_id = &params.user_id,
            "could not find user_id for site creation",
        );
        return not_found();
    }
    format::json(())
}

#[debug_handler]
pub async fn get_one(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let _ = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/sites/")
        .add("/", get(list))
        .add("/", post(add))
        .add("{id}", get(get_one))
}
