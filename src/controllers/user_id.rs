#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use crate::models::_entities::user_ids::{Entity, Model};
use crate::models::_entities::users;
use crate::views::user_id::{ListResponse, PaginationResponse};
use axum::debug_handler;
use axum::extract::Query;
use loco_rs::prelude::*;
use query::PaginationQuery;
use serde::{Deserialize, Serialize};

const fn default_page() -> u64 {
    1
}
const fn default_size() -> u64 {
    50
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryWithFilter {
    #[serde(default = "default_size")]
    size: u64,
    #[serde(default = "default_page")]
    page: u64,
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(
    auth: auth::JWT,
    Query(q): Query<QueryWithFilter>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let _ = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let pagination_query = PaginationQuery {
        page_size: q.size,
        page: q.page,
    };
    let pagination_user_ids =
        query::paginate(&ctx.db, Entity::find(), None, &pagination_query).await?;
    format::json(PaginationResponse::response(
        pagination_user_ids,
        &pagination_query,
    ))
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

#[debug_handler]
pub async fn add_one(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(ListResponse::new(&Model::create_new(&ctx.db).await?))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/user_ids/")
        .add("/", get(list))
        .add("/new", get(add_one))
        .add("/{id}", get(get_one))
}
