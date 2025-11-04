use twilight_model::application::command::{Command, CommandType};
use twilight_util::builder::command::CommandBuilder;

use crate::translation::Truncate;
use fate_internal_macro::app_command;

#[app_command]
fn campaign_new() -> Command {
    CommandBuilder::new(
        t!("campaign_new.command").c(32),
        t!("campaign_new.description").c(32),
        CommandType::ChatInput,
    )
    .build()
}
