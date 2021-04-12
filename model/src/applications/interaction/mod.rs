mod data;
mod kind;
mod response;

pub use data::InteractionData;
pub use kind::InteractionType;
pub use response::{InteractionResponse, InteractionResponseType};

use super::command::CommandData;
use crate::{
    guild::PartialMember,
    id::{ChannelId, GuildId, InteractionId},
};
use serde::{self, Deserialize, Deserializer, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Payload received when a user executes an interaction.
///
/// Each variant corresponds to `InteractionType` in the discord docs. Refer to
/// [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#interaction
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Interaction {
    /// Ping variant.
    Ping(PingInner),
    /// Application command variant.
    ApplicationCommand(ApplicationCommandInner),
}

impl Interaction {
    pub fn guild_id(&self) -> Option<GuildId> {
        match self {
            Interaction::Ping(_) => None,
            Interaction::ApplicationCommand(inner) => Some(inner.guild_id),
        }
    }
}

impl<'de> Deserialize<'de> for Interaction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let envelope = InteractionEnvelope::deserialize(deserializer)?;
        envelope.try_into().map_err(serde::de::Error::custom)
    }
}

/// Data present in an [`Interaction`] of type [`Ping`].
///
/// [`Ping`]: Interaction::Ping
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct PingInner {
    /// The id of the interaction
    pub id: InteractionId,
    #[serde(rename = "type")]
    /// The kind of the interaction
    pub kind: InteractionType,
    /// The token of the interaction
    pub token: String,
}

/// Data present in an [`Interaction`] of type [`ApplicationCommand`].
///
/// [`ApplicationCommand`]: Interaction::ApplicationCommand
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct ApplicationCommandInner {
    /// The guild the interaction was triggered from.
    pub guild_id: GuildId,
    /// The channel the interaction was triggered from.
    pub channel_id: ChannelId,
    /// The member that triggered the interaction.
    pub member: PartialMember,
    /// The data corresponding to the InteractionType.
    pub command_data: CommandData,
    /// The id of the interaction
    pub id: InteractionId,
    #[serde(rename = "type")]
    /// The kind of the interaction
    pub kind: InteractionType,
    /// The token of the interaction
    pub token: String,
}

impl<'a> TryFrom<InteractionEnvelope> for Interaction {
    type Error = InteractionEnvelopeParseError;

    fn try_from(envelope: InteractionEnvelope) -> Result<Self, Self::Error> {
        match envelope.kind {
            InteractionType::Ping => {
                let ping_inner = PingInner {
                    id: envelope.id,
                    kind: envelope.kind,
                    token: envelope.token,
                };

                Ok(Interaction::Ping(ping_inner))
            }
            InteractionType::ApplicationCommand => {
                let guild_id = match envelope.guild_id {
                    Some(id) => id,
                    None => return Err(Self::Error::MissingField("guild_id")),
                };

                let channel_id = match envelope.channel_id {
                    Some(id) => id,
                    None => return Err(Self::Error::MissingField("channel_id")),
                };

                let member = match envelope.member {
                    Some(m) => m,
                    None => return Err(Self::Error::MissingField("member")),
                };

                let command_data = match envelope.data {
                    Some(InteractionData::ApplicationCommand(cmd)) => cmd,
                    Some(_) => {
                        return Err(Self::Error::DataMismatch {
                            wanted: "command_data",
                            got: "other kind of data",
                        });
                    }
                    None => return Err(Self::Error::MissingField("data")),
                };

                Ok(Interaction::ApplicationCommand(ApplicationCommandInner {
                    guild_id,
                    channel_id,
                    member,
                    command_data,
                    id: envelope.id,
                    kind: envelope.kind,
                    token: envelope.token,
                }))
            }
        }
    }
}

/// Raw interaction payload received from Discord.
///
/// It is checked and parsed into an [`Interaction`].  Only used internally.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
struct InteractionEnvelope {
    id: InteractionId,
    #[serde(rename = "type")]
    kind: InteractionType,
    data: Option<InteractionData>,
    guild_id: Option<GuildId>,
    channel_id: Option<ChannelId>,
    member: Option<PartialMember>,
    token: String,
}

#[derive(Debug)]
enum InteractionEnvelopeParseError {
    DataMismatch {
        wanted: &'static str,
        got: &'static str,
    },
    MissingField(&'static str),
}

impl Display for InteractionEnvelopeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::DataMismatch { wanted, got } => {
                write!(f, "invalid data: wanted {} got {}", wanted, got)
            }
            Self::MissingField(s) => write!(f, "The field {} was missing", s),
        }
    }
}

impl std::error::Error for InteractionEnvelopeParseError {}

#[cfg(test)]
mod test {
    use super::ApplicationCommandInner;
    use crate::{
        applications::{
            command::{CommandData, CommandDataOption},
            interaction::{Interaction, InteractionType},
        },
        guild::{PartialMember, Permissions},
        id::UserId,
        user::{User, UserFlags},
    };

    #[test]
    fn test_interaction() {
        let json = r#"{
    "type": 2,
    "token": "A_UNIQUE_TOKEN",
    "member": {
        "user": {
            "id": "53908232506183680",
            "username": "Mason",
            "avatar": "a_d5efa99b3eeaa7dd43acca82f5692432",
            "discriminator": "1337",
            "public_flags": 131141
        },
        "roles": ["539082325061836999"],
        "premium_since": null,
        "permissions": "2147483647",
        "pending": false,
        "nick": null,
        "mute": false,
        "joined_at": "2017-03-13T19:19:14.040000+00:00",
        "is_pending": false,
        "deaf": false
    },
    "id": "786008729715212338",
    "guild_id": "290926798626357999",
    "data": {
        "options": [{
            "name": "cardname",
            "value": "The Gitrog Monster"
        }],
        "name": "cardsearch",
        "id": "771825006014889984"
    },
    "channel_id": "645027906669510667"
}"#;

        let expected = Interaction::ApplicationCommand(ApplicationCommandInner {
            guild_id: 290926798626357999.into(),
            channel_id: 645027906669510667.into(),
            member: PartialMember {
                user: Some(User {
                    id: UserId(53908232506183680),
                    name: "Mason".to_string(),
                    avatar: Some("a_d5efa99b3eeaa7dd43acca82f5692432".to_string()),
                    discriminator: 1337.to_string(),
                    public_flags: UserFlags::from_bits(131141),
                    bot: false,
                    email: None,
                    flags: None,
                    locale: None,
                    mfa_enabled: None,
                    premium_type: None,
                    system: None,
                    verified: None,
                }),
                roles: vec![539082325061836999.into()],
                permissions: Permissions::from_bits(2147483647),
                premium_since: None,
                nick: None,
                mute: false,
                joined_at: Some("2017-03-13T19:19:14.040000+00:00".to_string()),
                deaf: false,
            },
            command_data: CommandData {
                options: vec![CommandDataOption::String {
                    name: "cardname".to_string(),
                    value: "The Gitrog Monster".to_string(),
                }],
                name: "cardsearch".to_string(),
                id: 771825006014889984.into(),
            },
            id: 786008729715212338.into(),
            kind: InteractionType::ApplicationCommand,
            token: "A_UNIQUE_TOKEN".to_string(),
        });

        let actual = serde_json::from_str::<Interaction>(&json).unwrap();

        assert_eq!(expected, actual);
    }
}
