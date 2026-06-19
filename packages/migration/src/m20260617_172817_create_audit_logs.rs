use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AuditLogs::Table)
                    .col(
                        ColumnDef::new(AuditLogs::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AuditLogs::GuildId).uuid().not_null())
                    .col(ColumnDef::new(AuditLogs::ActorId).uuid().not_null())
                    .col(ColumnDef::new(AuditLogs::TargetId).uuid().null())
                    .col(ColumnDef::new(AuditLogs::ActionType).integer().not_null())
                    .col(ColumnDef::new(AuditLogs::Reason).string().null())
                    .col(ColumnDef::new(AuditLogs::Changes).json_binary().null())
                    .col(
                        ColumnDef::new(AuditLogs::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_audit_guild")
                            .from(AuditLogs::Table, AuditLogs::GuildId)
                            .to(Alias::new("guilds"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_audit_actor")
                            .from(AuditLogs::Table, AuditLogs::ActorId)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_audit_logs_guild_id")
                    .table(AuditLogs::Table)
                    .col(AuditLogs::GuildId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuditLogs::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum AuditLogs {
    Table,
    Id,
    GuildId,
    ActorId,
    TargetId,
    ActionType,
    Reason,
    Changes,
    CreatedAt,
}
