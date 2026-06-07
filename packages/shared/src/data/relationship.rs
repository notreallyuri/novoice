use crate::data::{UserId, user::UserPublic};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipStatus {
    None,
    Friend,
    Blocked,
    PendingIncoming,
    PendingOutgoing,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRelationship {
    pub id: UserId,
    pub user: UserPublic,
    pub status: RelationshipStatus,
    pub since: String,
}
