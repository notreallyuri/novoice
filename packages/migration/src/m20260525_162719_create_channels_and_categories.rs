use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .col(
                        ColumnDef::new(Categories::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Categories::GuildId).uuid().not_null())
                    .col(ColumnDef::new(Categories::Name).string().not_null())
                    .col(ColumnDef::new(Categories::Position).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_category_guild")
                            .from(Categories::Table, Categories::GuildId)
                            .to(Alias::new("guilds"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Channels::Table)
                    .col(ColumnDef::new(Channels::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Channels::GuildId).uuid().not_null())
                    .col(ColumnDef::new(Channels::Name).string().not_null())
                    .col(ColumnDef::new(Channels::Position).integer().not_null())
                    .col(ColumnDef::new(Channels::CategoryId).uuid().null())
                    .col(ColumnDef::new(Channels::Kind).integer().not_null())
                    .col(ColumnDef::new(Channels::Mode).integer().null())
                    .col(ColumnDef::new(Channels::UserLimit).integer().null())
                    .col(ColumnDef::new(Channels::Bitrate).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_channel_guild")
                            .from(Channels::Table, Channels::GuildId)
                            .to(Alias::new("guilds"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_channel_category")
                            .from(Channels::Table, Channels::CategoryId)
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Channels::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Categories {
    Table,
    Id,
    GuildId,
    Name,
    Position,
}
#[derive(DeriveIden)]
enum Channels {
    Table,
    Id,
    GuildId,
    Name,
    Position,
    CategoryId,
    Kind,
    Mode,
    UserLimit,
    Bitrate,
}
