use crate::{id::RoleId, user::User, guild::Permissions};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PartialMember {
    pub deaf: bool,
    pub joined_at: Option<String>,
    pub mute: bool,
    pub nick: Option<String>,
    pub permissions: Option<Permissions>,
    pub roles: Vec<RoleId>,
    pub user: Option<User>
}

#[cfg(test)]
mod tests {
    use super::{PartialMember, RoleId};
    use serde_test::Token;

    #[test]
    fn test_partial_member() {
        let value = PartialMember {
            deaf: false,
            joined_at: Some("timestamp".to_owned()),
            mute: true,
            nick: Some("a nickname".to_owned()),
            roles: vec![RoleId(1)],
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "PartialMember",
                    len: 5,
                },
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("joined_at"),
                Token::Some,
                Token::Str("timestamp"),
                Token::Str("mute"),
                Token::Bool(true),
                Token::Str("nick"),
                Token::Some,
                Token::Str("a nickname"),
                Token::Str("roles"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("1"),
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
