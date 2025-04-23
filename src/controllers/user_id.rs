#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use axum::debug_handler;
use crate::models::_entities::user_ids::{Entity, Model, ActiveModel};
use crate::models::_entities::users;
use crate::views::user_id::NewResponse;

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let _ = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(Entity::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn get_one(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>
) -> Result<Response> {
    let _ = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(load_item(&ctx, id).await?)
}

#[debug_handler]
pub async fn add_one(State(ctx): State<AppContext>) -> Result<Response> {
    let mut item: ActiveModel = Default::default();
    item.user_id = Set(Uuid::new_v4());
    let user_id = item.insert(&ctx.db).await?;
    format::json(NewResponse::new(&user_id))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/user_ids/")
        .add("/", get(list))
        .add("/new", get(add_one))
        .add("/{id}", get(get_one))
}
