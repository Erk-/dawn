use super::InteractionError;
use crate::request::prelude::*;
use twilight_model::{id::{ApplicationId, GuildId}, applications::command::GuildCommandPermissions};

pub struct GetGuildCommandsPermissions<'a> {
    application_id: ApplicationId,
    guild_id: GuildId,
    fut: Option<Pending<'a, Vec<GuildCommandPermissions>>>,
    http: &'a Client
}

impl<'a> GetGuildCommandsPermissions<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Option<ApplicationId>,
        guild_id: GuildId,
    ) -> Result<Self, InteractionError> {
        let application_id = application_id.ok_or(InteractionError::ApplicationIdNotPresent)?;

        Ok(Self {
            application_id,
            guild_id,
            fut: None,
            http,
        })
    }

    fn start(&mut self) -> Result<()> {
        let req = Request::from(Route::GetGuildCommandsPermissions {
            application_id: self.application_id.0,
            guild_id: self.guild_id.0
        });
        self.fut.replace(Box::pin(self.http.request(req)));

        Ok(())
    }
}

poll_req!(GetGuildCommandsPermissions<'_>, Vec<GuildCommandPermissions>);
