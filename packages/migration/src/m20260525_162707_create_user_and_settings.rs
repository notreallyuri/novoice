use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .col(ColumnDef::new(Users::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(Users::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Users::DisplayName).string().not_null())
                    .col(ColumnDef::new(Users::BannerUrl).string().null())
                    .col(ColumnDef::new(Users::AvatarUrl).string().null())
                    .col(ColumnDef::new(Users::Bio).string().null())
                    .col(ColumnDef::new(Users::ProfileColor).string().null())
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserAccounts::Table)
                    .col(
                        ColumnDef::new(UserAccounts::UserId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserAccounts::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(UserAccounts::Verified)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(UserAccounts::PasswordHash)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserAccounts::TwoFactorEnabled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(UserAccounts::TwoFactorSecret)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(UserAccounts::RecoveryKeys)
                            .array(ColumnType::String(StringLen::None))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserAccounts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserAccounts::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_accounts_user")
                            .from(UserAccounts::Table, UserAccounts::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserSettings::Table)
                    .col(
                        ColumnDef::new(UserSettings::UserId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserSettings::ThemeDarkMode)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserSettings::ThemeColor)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserSettings::ThemeRounding)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserSettings::ThemeSpacing)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserSettings::NotificationActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_settings_user")
                            .from(UserSettings::Table, UserSettings::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Relationships::Table)
                    .col(ColumnDef::new(Relationships::UserId).uuid().not_null())
                    .col(ColumnDef::new(Relationships::TargetId).uuid().not_null())
                    .col(ColumnDef::new(Relationships::Status).integer().not_null())
                    .col(ColumnDef::new(Relationships::Since).timestamp().not_null())
                    .primary_key(
                        Index::create()
                            .col(Relationships::UserId)
                            .col(Relationships::TargetId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_rel_user")
                            .from(Relationships::Table, Relationships::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_rel_target")
                            .from(Relationships::Table, Relationships::TargetId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Relationships::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserSettings::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserAccounts::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    DisplayName,
    BannerUrl,
    AvatarUrl,
    Bio,
    ProfileColor,
    CreatedAt,
    UpdatedAt,
}
#[derive(DeriveIden)]
enum UserAccounts {
    Table,
    UserId,
    Email,
    Verified,
    PasswordHash,
    TwoFactorEnabled,
    TwoFactorSecret,
    RecoveryKeys,
    CreatedAt,
    UpdatedAt,
}
#[derive(DeriveIden)]
enum UserSettings {
    Table,
    UserId,
    ThemeDarkMode,
    ThemeColor,
    ThemeRounding,
    ThemeSpacing,
    NotificationActive,
}
#[derive(DeriveIden)]
enum Relationships {
    Table,
    UserId,
    TargetId,
    Status,
    Since,
}
