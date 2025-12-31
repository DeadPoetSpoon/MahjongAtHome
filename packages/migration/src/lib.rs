pub use sea_orm_migration::prelude::*;

mod m20251230_134744_create_role;
mod m20251230_134748_create_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251230_134744_create_role::Migration),
            Box::new(m20251230_134748_create_user::Migration),
        ]
    }
}
