use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .col(ColumnDef::new(Roles::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Roles::GuildId).uuid().not_null())
                    .col(ColumnDef::new(Roles::Name).string().not_null())
                    .col(
                        ColumnDef::new(Roles::Permissions)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Roles::Color).integer().null())
                    .col(
                        ColumnDef::new(Roles::Hoist)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Roles::Position)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_guild")
                            .from(Roles::Table, Roles::GuildId)
                            .to(Alias::new("guilds"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(GuildMembersRoles::Table)
                    .col(ColumnDef::new(GuildMembersRoles::RoleId).uuid().not_null())
                    .col(
                        ColumnDef::new(GuildMembersRoles::GuildMemberId)
                            .uuid()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(GuildMembersRoles::RoleId)
                            .col(GuildMembersRoles::GuildMemberId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_gmr_role")
                            .from(GuildMembersRoles::Table, GuildMembersRoles::RoleId)
                            .to(Roles::Table, Roles::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_gmr_member")
                            .from(GuildMembersRoles::Table, GuildMembersRoles::GuildMemberId)
                            .to(Alias::new("guild_members"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CategoryOverrides::Table)
                    .col(
                        ColumnDef::new(CategoryOverrides::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CategoryOverrides::CategoryId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CategoryOverrides::TargetId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CategoryOverrides::TargetType)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CategoryOverrides::AllowBits)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(CategoryOverrides::DenyBits)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_category_override")
                            .from(CategoryOverrides::Table, CategoryOverrides::CategoryId)
                            .to(Alias::new("categories"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ChannelOverrides::Table)
                    .col(
                        ColumnDef::new(ChannelOverrides::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ChannelOverrides::ChannelId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ChannelOverrides::TargetId).uuid().not_null())
                    .col(
                        ColumnDef::new(ChannelOverrides::TargetType)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ChannelOverrides::AllowBits)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(ChannelOverrides::DenyBits)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_channel_override")
                            .from(ChannelOverrides::Table, ChannelOverrides::ChannelId)
                            .to(Alias::new("channels"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ChannelOverrides::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(CategoryOverrides::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GuildMembersRoles::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Roles::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Roles {
    Table,
    Id,
    GuildId,
    Name,
    Permissions,
    Color,
    Hoist,
    Position,
}

#[derive(DeriveIden)]
enum GuildMembersRoles {
    Table,
    RoleId,
    GuildMemberId,
}

#[derive(DeriveIden)]
enum CategoryOverrides {
    Table,
    Id,
    CategoryId,
    TargetId,
    TargetType,
    AllowBits,
    DenyBits,
}

#[derive(DeriveIden)]
enum ChannelOverrides {
    Table,
    Id,
    ChannelId,
    TargetId,
    TargetType,
    AllowBits,
    DenyBits,
}
