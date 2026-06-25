use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Guilds::Table)
                    .col(ColumnDef::new(Guilds::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Guilds::Name).string().not_null())
                    .col(ColumnDef::new(Guilds::OwnerId).uuid().not_null())
                    .col(ColumnDef::new(Guilds::IconUrl).string().null())
                    .col(ColumnDef::new(Guilds::BannerUrl).string().null())
                    .col(ColumnDef::new(Guilds::DefaultChannelId).uuid().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_guild_owner")
                            .from(Guilds::Table, Guilds::OwnerId)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GuildMembers::Table)
                    .col(
                        ColumnDef::new(GuildMembers::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(GuildMembers::UserId).uuid().not_null())
                    .col(ColumnDef::new(GuildMembers::GuildId).uuid().not_null())
                    .col(
                        ColumnDef::new(GuildMembers::IdentityShowGlobalUsername)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(GuildMembers::IdentityDisplayName)
                            .string()
                            .null(),
                    )
                    .col(ColumnDef::new(GuildMembers::IdentityAvatar).string().null())
                    .col(ColumnDef::new(GuildMembers::IdentityBanner).string().null())
                    .col(ColumnDef::new(GuildMembers::IdentityBio).string().null())
                    .col(
                        ColumnDef::new(GuildMembers::JoinedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_member_user")
                            .from(GuildMembers::Table, GuildMembers::UserId)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_member_guild")
                            .from(GuildMembers::Table, GuildMembers::GuildId)
                            .to(Guilds::Table, Guilds::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GuildBans::Table)
                    .col(ColumnDef::new(GuildBans::GuildId).uuid().not_null())
                    .col(ColumnDef::new(GuildBans::UserId).uuid().not_null())
                    .col(ColumnDef::new(GuildBans::Reason).string().null())
                    .col(ColumnDef::new(GuildBans::BannedBy).uuid().not_null())
                    .col(
                        ColumnDef::new(GuildBans::ExpiresAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(GuildBans::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk_guild_bans")
                            .col(GuildBans::GuildId)
                            .col(GuildBans::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_guild_ban_guild")
                            .from(GuildBans::Table, GuildBans::GuildId)
                            .to(Guilds::Table, Guilds::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_guild_ban_user")
                            .from(GuildBans::Table, GuildBans::UserId)
                            .to(Alias::new("users"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_guild_ban_admin")
                            .from(GuildBans::Table, GuildBans::BannedBy)
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
            .drop_table(Table::drop().table(GuildBans::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GuildMembers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Guilds::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Guilds {
    Table,
    Id,
    Name,
    OwnerId,
    IconUrl,
    BannerUrl,
    DefaultChannelId,
}
#[derive(DeriveIden)]
enum GuildMembers {
    Table,
    Id,
    UserId,
    GuildId,
    IdentityShowGlobalUsername,
    IdentityDisplayName,
    IdentityAvatar,
    IdentityBanner,
    IdentityBio,
    JoinedAt,
}
#[derive(DeriveIden)]
enum GuildBans {
    Table,
    GuildId,
    UserId,
    Reason,
    BannedBy,
    ExpiresAt,
    CreatedAt,
}
