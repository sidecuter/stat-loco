pub use super::_entities::user_ids::{ActiveModel, Entity, Model};
use crate::models::_entities::user_ids;
use loco_rs::model;
use loco_rs::model::{ModelError, ModelResult};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, DatabaseTransaction};
pub type UserIds = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

// implement your read-oriented logic here
impl Model {
    /// finds uuid by the provided id
    ///
    /// # Errors
    ///
    /// When could not find `user_id` by the given id or DB query error
    pub async fn find_id_by_uuid(db: &DatabaseTransaction, user_id: &str) -> ModelResult<i32> {
        let parse_uuid = Uuid::parse_str(user_id).map_err(|e| ModelError::Any(e.into()))?;
        let user_id = UserIds::find()
            .filter(model::query::condition().eq(user_ids::Column::UserId, parse_uuid))
            .one(db)
            .await?;
        user_id
            .ok_or(ModelError::EntityNotFound)
            .map(|user_id| user_id.id)
    }

    /// creates new `user_ids` entry in database
    ///
    /// # Errors
    ///
    /// On DB error
    pub async fn create_new(db: &DatabaseConnection) -> ModelResult<Self> {
        let item = ActiveModel {
            user_id: ActiveValue::Set(Uuid::new_v4()),
            ..Default::default()
        };
        let user_id = item.insert(db).await?;
        Ok(user_id)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
