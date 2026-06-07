use crate::data::{
    relationship::UserRelationship,
    user::{User, UserPublic},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event")]
pub enum UserServerEvents {
    Ready,
    IdentityValidated { user: Box<User> },
    PresenceUpdate { user: Box<UserPublic> },
    RelationshipUpdate { relationship: Box<UserRelationship> },
}
