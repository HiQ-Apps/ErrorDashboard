use sea_orm_migration::prelude::*;


use super::m20231013_200027_create_namespace_table::Namespaces;

#[derive(DeriveIden)]
pub enum Errors {
    Table,
    Id,
    StatusCode,
    UserAffected,
    Path,
    Line,
    Message,
    Namespace,
    StackTrace,
    Resolved,
    CreatedAt,
    UpdatedAt
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Errors::Table)
                    .if_not_exists()
                        .col(ColumnDef::new(Errors::Id).uuid().primary_key())
                        .col(ColumnDef::new(Errors::StatusCode).small_integer().not_null())
                        .col(ColumnDef::new(Errors::UserAffected).string().not_null())
                        .col(ColumnDef::new(Errors::Path).string().not_null())
                        .col(ColumnDef::new(Errors::Line).integer().not_null())
                        .col(ColumnDef::new(Errors::Message).string().not_null())
                        .col(ColumnDef::new(Errors::Namespace).uuid().not_null())
                        .foreign_key(
                            ForeignKey::create()
                                .name("fk_error_namespace")
                                .from(Errors::Table, Errors::Namespace)
                                .to(Namespaces::Table, Namespaces::Id)
                                .on_delete(ForeignKeyAction::Cascade)
                                .on_update(ForeignKeyAction::Cascade),
                        )
                        .col(ColumnDef::new(Errors::StackTrace).string().not_null())
                        .col(ColumnDef::new(Errors::Resolved).boolean().not_null())
                        .col(ColumnDef::new(Errors::CreatedAt).date_time().not_null())
                        .col(ColumnDef::new(Errors::UpdatedAt).date_time().not_null())
                        .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Errors::Table).to_owned())
            .await
    }
}
