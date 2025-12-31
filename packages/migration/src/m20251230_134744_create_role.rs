use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(RoleType::Type)
                    .values([
                        RoleType::SuperAdmin,
                        RoleType::Admin,
                        RoleType::User,
                        RoleType::Guest,
                    ])
                    .to_owned(),
            )
            .await?;
        manager
            .create_type(
                Type::create()
                    .as_enum(RoleActionType::Type)
                    .values([RoleActionType::Signup])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(pk_auto(Role::Id))
                    .col(ColumnDef::new(Role::Type).custom(RoleType::Type).not_null())
                    .col(
                        ColumnDef::new(Role::AllowAction)
                            .custom(RoleActionType::Type)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(RoleType::Type).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(RoleActionType::Type).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Role {
    Table,
    Id,
    Type,
    AllowAction,
}

#[derive(Iden)]
pub enum RoleType {
    #[iden = "role_type"]
    Type,
    SuperAdmin,
    Admin,
    User,
    Guest,
}
#[derive(Iden)]
pub enum RoleActionType {
    #[iden = "role_action_type"]
    Type,
    Signup,
}
