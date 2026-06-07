use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Default, Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[serde(rename_all = "lowercase")]
pub enum DbThemeSpacing {
    #[default]
    #[sea_orm(num_value = 0)]
    Default,
    #[sea_orm(num_value = 1)]
    Compact,
    #[sea_orm(num_value = 2)]
    Comfortable,
}

#[derive(
    Default, Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[serde(rename_all = "lowercase")]
pub enum DbThemeRounding {
    #[default]
    #[sea_orm(num_value = 0)]
    Default,
    #[sea_orm(num_value = 1)]
    Comfortable,
    #[sea_orm(num_value = 2)]
    Full,
}

#[derive(
    Default, Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[serde(rename_all = "lowercase")]
pub enum DbThemeDarkMode {
    #[default]
    #[sea_orm(num_value = 0)]
    System,
    #[sea_orm(num_value = 1)]
    Light,
    #[sea_orm(num_value = 2)]
    Dark,
}

#[derive(
    Default, Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[serde(rename_all = "lowercase")]
pub enum DbThemeColor {
    #[default]
    #[sea_orm(num_value = 0)]
    Default,
    #[sea_orm(num_value = 1)]
    Strawberry,
    #[sea_orm(num_value = 2)]
    Blueberry,
}

#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user_settings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,
    pub theme_dark_mode: DbThemeDarkMode,
    pub theme_color: DbThemeColor,
    pub theme_rounding: DbThemeRounding,
    pub theme_spacing: DbThemeSpacing,
    pub notification_active: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
