use entity::role::Model as RoleModel;
use shared::data::{RoleId, guild::Role};

use crate::core::mappers::FromDomain;

impl FromDomain<RoleModel> for Role {
    fn from_domain(value: RoleModel) -> Self {
        Role {
            id: RoleId(value.id),
            name: value.name,
            color: value.color,
            hoist: value.hoist,
            position: value.position,
            permissions: value.permissions,
        }
    }
}
