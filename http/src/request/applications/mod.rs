use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

mod create_guild_command;
mod delete_guild_command;
mod get_guild_commands;
mod set_guild_commands;
mod update_guild_command;

mod create_global_command;
mod delete_global_command;
mod get_global_commands;
mod set_global_commands;
mod update_global_command;

mod get_guild_commands_permission;
mod get_guild_command_permission;
mod update_guild_command_permission;
mod set_guild_commands_permission;

mod interaction_callback;

pub use create_guild_command::CreateGuildCommand;
pub use delete_guild_command::DeleteGuildCommand;
pub use get_guild_commands::GetGuildCommands;
pub use set_guild_commands::SetGuildCommands;
pub use update_guild_command::UpdateGuildCommand;

pub use create_global_command::CreateGlobalCommand;
pub use delete_global_command::DeleteGlobalCommand;
pub use get_global_commands::GetGlobalCommands;
pub use set_global_commands::SetGlobalCommands;
pub use update_global_command::UpdateGlobalCommand;

pub use get_guild_commands_permission::GetGuildCommandsPermissions;
pub use get_guild_command_permission::GetGuildCommandPermissions;
pub use update_guild_command_permission::UpdateGuildCommandPermissions;
pub use set_guild_commands_permission::SetGuildCommandsPermissions;

pub use interaction_callback::InteractionCallback;

#[derive(Debug)]
#[non_exhaustive]
pub enum InteractionError {
    ApplicationIdNotPresent,
}

impl Display for InteractionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::ApplicationIdNotPresent => f.write_str("application id not present"),
        }
    }
}

impl Error for InteractionError {}
