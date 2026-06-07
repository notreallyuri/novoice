use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PresencePresets::Table)
                    .col(
                        ColumnDef::new(PresencePresets::Id)
                            .uuid()
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PresencePresets::UserId).uuid().not_null())
                    .col(ColumnDef::new(PresencePresets::Label).string().not_null())
                    .col(
                        ColumnDef::new(PresencePresets::IconKind)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PresencePresets::IconValue).string())
                    .col(
                        ColumnDef::new(PresencePresets::TimerKind)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PresencePresets::TimerSeconds).big_integer())
                    .col(
                        ColumnDef::new(PresencePresets::PresetKind)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PresencePresets::ProcessName).string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_presence_preset_user")
                            .from(PresencePresets::Table, PresencePresets::UserId)
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
            .drop_table(Table::drop().table(PresencePresets::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum PresencePresets {
    Table,
    Id,
    UserId,
    Label,
    IconKind,
    IconValue,
    TimerKind,
    TimerSeconds,
    PresetKind,
    ProcessName,
}
