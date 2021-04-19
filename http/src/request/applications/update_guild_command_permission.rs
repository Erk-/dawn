use super::InteractionError;
use crate::request::prelude::*;
use twilight_model::{applications::command::ApplicationCommandPermissions, id::{ApplicationId, GuildId, CommandId}};
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct UpdatePermissionsFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ApplicationCommandPermissions>>
}

pub struct UpdateGuildCommandPermissions<'a> {
    fields: UpdatePermissionsFields,
    application_id: ApplicationId,
    command_id: CommandId,
    guild_id: GuildId,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
}

impl<'a> UpdateGuildCommandPermissions<'a> {
    pub(crate) fn new(http: &'a Client,
        application_id: Option<ApplicationId>,
        guild_id: GuildId,
        command_id: CommandId,
    ) -> Result<Self, InteractionError> {
        let application_id = application_id.ok_or(InteractionError::ApplicationIdNotPresent)?;

        Ok(Self {
            application_id,
            command_id,
            guild_id,
            fut: None,
            fields: UpdatePermissionsFields::default(),
            http
        })
    }

    pub fn permissions(mut self, permissions: Vec<ApplicationCommandPermissions>) -> Self {
        self.fields.permissions.replace(permissions);

        self
    }

    fn start(&mut self) -> Result<()> {
        let req = Request::from((
            crate::json_to_vec(&self.fields)?,
            Route::UpdateGuildCommandPermissions {
                application_id: self.application_id.0,
                command_id: self.command_id.0,
                guild_id: self.guild_id.0
            }
        ));
        self.fut.replace(Box::pin(self.http.verify(req)));

        Ok(())
    }
}

poll_req!(UpdateGuildCommandPermissions<'_>, ());