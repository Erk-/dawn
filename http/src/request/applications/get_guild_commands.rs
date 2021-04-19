use super::InteractionError;
use crate::request::prelude::*;
use twilight_model::{
    applications::command::Command,
    id::{ApplicationId, GuildId},
};

/// Fetch all commands for a guild, by ID.
pub struct GetGuildCommands<'a> {
    application_id: ApplicationId,
    guild_id: GuildId,
    fut: Option<Pending<'a, Vec<Command>>>,
    http: &'a Client,
}

impl<'a> GetGuildCommands<'a> {
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
        let req = Request::from(Route::GetGuildCommands {
            application_id: self.application_id.0,
            guild_id: self.guild_id.0,
        });
        self.fut.replace(Box::pin(self.http.request(req)));

        Ok(())
    }
}

poll_req!(GetGuildCommands<'_>, Vec<Command>);