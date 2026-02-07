pub use sea_orm_migration::prelude::*;

mod m20251230_134744_create_role;
mod m20251230_134748_create_user;
mod m20260105_141340_create_user_info;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251230_134744_create_role::Migration),
            Box::new(m20251230_134748_create_user::Migration),
            Box::new(m20260105_141340_create_user_info::Migration),
        ]
    }
}
