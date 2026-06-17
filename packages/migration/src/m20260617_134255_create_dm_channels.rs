use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DmChannels::Table)
                    .col(
                        ColumnDef::new(DmChannels::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(DmChannels::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(DmChannelMembers::Table)
                    .col(
                        ColumnDef::new(DmChannelMembers::ChannelId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(DmChannelMembers::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(DmChannelMembers::IsOpen)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(DmChannelMembers::JoinedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(DmChannelMembers::ChannelId)
                            .col(DmChannelMembers::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_dm_member_channel")
                            .from(DmChannelMembers::Table, DmChannelMembers::ChannelId)
                            .to(DmChannels::Table, DmChannels::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_dm_member_user")
                            .from(DmChannelMembers::Table, DmChannelMembers::UserId)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DmChannelMembers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(DmChannels::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum DmChannels {
    Table,
    Id,
    CreatedAt,
}

#[derive(DeriveIden)]
enum DmChannelMembers {
    Table,
    ChannelId,
    UserId,
    IsOpen,
    JoinedAt,
}
