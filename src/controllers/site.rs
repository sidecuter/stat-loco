#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use axum::debug_handler;

use crate::models::_entities::sites::{ActiveModel, Entity, Model};
use crate::models::_entities::users;
use crate::models::sites::AddParams;

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(
    auth: auth::JWT,
    State(ctx): State<AppContext>
) -> Result<Response> {
    let _ = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(Entity::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(State(ctx): State<AppContext>, Json(params): Json<AddParams>) -> Result<Response> {
    let res = ActiveModel::create_with_uuid(&ctx.db, &params).await;
    if let Err(err) = res {
        tracing::info!(
                message = err.to_string(),
                user_id = &params.user_id,
                "could not find user_id for site creation",
            );
        return not_found()
    }
    format::json(())
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

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/sites/")
        .add("/", get(list))
        .add("/", post(add))
        .add("{id}", get(get_one))
}
