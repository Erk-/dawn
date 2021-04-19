use crate::id::{ApplicationId, CommandId, GenericId, GuildId, RoleId, UserId};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// The struct representing permissions of a command.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#guildapplicationcommandpermissions
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildCommandPermissions {
    /* TODO: Different struct in http for builder purposes (same as Command) */
    pub id: CommandId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    pub permissions: Vec<ApplicationCommandPermissions>,
}

/// Specifies what type of permission is enabled or disabled for a specific user or role
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#applicationcommandpermissions
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ApplicationCommandPermissions {
    User { id: UserId, permission: bool },
    Role { id: RoleId, permission: bool },
}

impl<'de> Deserialize<'de> for ApplicationCommandPermissions {
    fn deserialize<D>(deserializer: D) -> Result<ApplicationCommandPermissions, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(ApplicationCommandPermissionsEnvelope::deserialize(deserializer)?.into())
    }
}

impl Serialize for ApplicationCommandPermissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (kind, id, permission) = match self {
            ApplicationCommandPermissions::Role { id, permission } => {
                (ApplicationCommandPermissionType::Role, id.0, *permission)
            }
            ApplicationCommandPermissions::User { id, permission } => {
                (ApplicationCommandPermissionType::User, id.0, *permission)
            }
        };
        ApplicationCommandPermissionsEnvelope {
            kind,
            id: GenericId(id),
            permission,
        }
        .serialize(serializer)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
struct ApplicationCommandPermissionsEnvelope {
    #[serde(rename = "type")]
    pub kind: ApplicationCommandPermissionType,
    pub id: GenericId,
    pub permission: bool,
}

#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
#[repr(u8)]
pub enum ApplicationCommandPermissionType {
    Role = 1,
    User = 2,
}

impl From<ApplicationCommandPermissionsEnvelope> for ApplicationCommandPermissions {
    fn from(envelope: ApplicationCommandPermissionsEnvelope) -> Self {
        match envelope.kind {
            ApplicationCommandPermissionType::Role => Self::Role {
                id: RoleId(envelope.id.0),
                permission: envelope.permission,
            },
            ApplicationCommandPermissionType::User => Self::User {
                id: UserId(envelope.id.0),
                permission: envelope.permission,
            },
        }
    }
}
