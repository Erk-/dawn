use super::InteractionError;
use crate::request::prelude::*;
use twilight_model::{id::{ApplicationId, GuildId, CommandId}, applications::command::GuildCommandPermissions};

pub struct GetGuildCommandPermissions<'a> {
    application_id: ApplicationId,
    guild_id: GuildId,
    command_id: CommandId,
    fut: Option<Pending<'a, GuildCommandPermissions>>,
    http: &'a Client
}

impl<'a> GetGuildCommandPermissions<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Option<ApplicationId>,
        guild_id: GuildId,
        command_id: CommandId
    ) -> Result<Self, InteractionError> {
        let application_id = application_id.ok_or(InteractionError::ApplicationIdNotPresent)?;

        Ok(Self {
            application_id,
            guild_id,
            command_id,
            fut: None,
            http,
        })
    }

    fn start(&mut self) -> Result<()> {
        let req = Request::from(Route::GetGuildCommandPermissions {
            application_id: self.application_id.0,
            guild_id: self.guild_id.0,
            command_id: self.command_id.0
        });
        self.fut.replace(Box::pin(self.http.request(req)));

        Ok(())
    }
}

poll_req!(GetGuildCommandPermissions<'_>, GuildCommandPermissions);
