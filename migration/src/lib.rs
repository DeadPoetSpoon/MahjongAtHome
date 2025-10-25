pub use sea_orm_migration::prelude::*;

mod m20251025_081315_create_role;
mod m20251025_083756_create_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251025_081315_create_role::Migration),
            Box::new(m20251025_083756_create_user::Migration),
        ]
    }
}
