use super::InteractionError;
use crate::request::prelude::*;
use twilight_model::{
    applications::command::GuildCommandPermissions,
    id::{ApplicationId, GuildId},
};

pub struct SetGuildCommandsPermissions<'a> {
    commands_permissions: Vec<GuildCommandPermissions>,
    application_id: ApplicationId,
    guild_id: GuildId,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
}

impl<'a> SetGuildCommandsPermissions<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Option<ApplicationId>,
        guild_id: GuildId,
        commands_permissions: Vec<GuildCommandPermissions>,
    ) -> Result<Self, InteractionError> {
        let application_id = application_id.ok_or(InteractionError::ApplicationIdNotPresent)?;

        Ok(Self {
            commands_permissions,
            application_id,
            guild_id,
            fut: None,
            http,
        })
    }

    fn start(&mut self) -> Result<()> {
        let req = Request::from((
            crate::json_to_vec(&self.commands_permissions)?,
            Route::SetGuildCommandsPermissions {
                application_id: self.application_id.0,
                guild_id: self.guild_id.0,
            },
        ));
        self.fut.replace(Box::pin(self.http.verify(req)));

        Ok(())
    }
}

poll_req!(SetGuildCommandsPermissions<'_>, ());
