pub use super::_entities::sites::{ActiveModel, Entity, Model};
use crate::models::user_ids;
use crate::validators::validate_uuid;
use loco_rs::model::ModelResult;
use loco_rs::prelude::Validate;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, TransactionTrait};
use serde::{Deserialize, Serialize};
pub type Sites = Entity;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AddParams {
    pub endpoint: Option<String>,
    #[validate(custom(function = "validate_uuid"))]
    pub user_id: String,
}

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
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {
    /// Creates model with specified uuid.
    ///
    /// # Errors
    /// - Returns an error if database query fails
    pub async fn create_with_uuid(db: &DatabaseConnection, params: &AddParams) -> ModelResult<()> {
        let txn = db.begin().await?;

        let uid = user_ids::Model::find_id_by_uuid(&txn, &params.user_id).await?;

        let _ = Self {
            user_id: ActiveValue::Set(uid),
            endpoint: ActiveValue::Set(params.endpoint.clone()),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        txn.commit().await?;

        Ok(())
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
