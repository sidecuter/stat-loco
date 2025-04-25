#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250423_101253_user_ids;
mod m20250423_144222_sites;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250423_101253_user_ids::Migration),
            Box::new(m20250423_144222_sites::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
