use crate::{
    guild::GuildStatus,
    id::{ApplicationId, GuildId},
    user::{CurrentUser, UserFlags},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PartialApplication {
    pub flags: UserFlags,
    pub id: ApplicationId,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Ready {
    pub application: PartialApplication,
    #[serde(with = "serde_mappable_seq")]
    pub guilds: HashMap<GuildId, GuildStatus>,
    pub session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<[u64; 2]>,
    pub user: CurrentUser,
    #[serde(rename = "v")]
    pub version: u64,
}
