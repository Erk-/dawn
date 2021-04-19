use super::InteractionError;
use crate::request::prelude::*;
use twilight_model::{
    applications::command::{Command, CommandOption},
    id::ApplicationId,
};

/// Create a new global command.
///
/// The name must be between 3 and 32 characters in length, and the description
/// must be between 1 and 100 characters in length. Creating a command with the
/// same name as an already-existing global command will overwwrite the old
/// command. See [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/interactions/slash-commands#create-global-application-command
pub struct CreateGlobalCommand<'a> {
    command: Command,
    application_id: ApplicationId,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
}

impl<'a> CreateGlobalCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Option<ApplicationId>,
        name: String,
        description: String,
    ) -> Result<Self, InteractionError> {
        let application_id = application_id.ok_or(InteractionError::ApplicationIdNotPresent)?;

        Ok(Self {
            command: Command {
                id: None,
                application_id: Some(application_id),
                name,
                description,
                options: vec![],
                default_permission: true
            },
            application_id,
            fut: None,
            http,
        })
    }

    /// Edit the default permission of the command
    pub fn default_permission(mut self, default_permission: bool) -> Self {
        self.command.default_permission = default_permission;

        self
    }

    /// Add a command option.
    pub fn push_command_option(mut self, option: CommandOption) -> Self {
        self.command.options.push(option);

        self
    }

    fn start(&mut self) -> Result<()> {
        let req = Request::from((
            crate::json_to_vec(&self.command)?,
            Route::CreateGlobalCommand {
                application_id: self.application_id.0,
            },
        ));
        self.fut.replace(Box::pin(self.http.verify(req)));

        Ok(())
    }
}

poll_req!(CreateGlobalCommand<'_>, ());
