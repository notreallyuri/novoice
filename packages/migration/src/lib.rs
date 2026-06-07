pub use sea_orm_migration::prelude::*;

mod m20260525_162707_create_user_and_settings;
mod m20260525_162712_create_guilds;
mod m20260525_162719_create_channels_and_categories;
mod m20260525_162724_create_invites;
mod m20260525_215659_create_roles;
mod m20260531_215949_create_presence_presets;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260525_162707_create_user_and_settings::Migration),
            Box::new(m20260525_162712_create_guilds::Migration),
            Box::new(m20260525_162719_create_channels_and_categories::Migration),
            Box::new(m20260525_162724_create_invites::Migration),
            Box::new(m20260525_215659_create_roles::Migration),
            Box::new(m20260531_215949_create_presence_presets::Migration),
        ]
    }
}
