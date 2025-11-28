use anyhow::Result;
use twilight_model::application::{
    command::{Command, CommandType},
    interaction::InteractionContextType,
};
use twilight_util::builder::command::{CommandBuilder, StringBuilder};

#[derive(Debug)]
pub struct Slash(pub fn() -> Result<Command>);

inventory::collect!(Slash);

#[must_use]
pub fn get_commands() -> Vec<Command> {
    inventory::iter::<Slash>
        .into_iter()
        .map(|builder| (builder.0)())
        .filter_map(Result::ok)
        .collect()
}

pub trait CommandLocalize {
    fn init_localize(
        global_name: impl Into<String>,
        global_description: impl Into<String>,
        local_names: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
        local_descriptions: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self;
}

impl CommandLocalize for CommandBuilder {
    fn init_localize(
        global_name: impl Into<String>,
        global_description: impl Into<String>,
        local_names: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
        local_descriptions: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        Self::new(global_name, global_description, CommandType::ChatInput)
            .name_localizations(local_names)
            .description_localizations(local_descriptions)
            .contexts([
                InteractionContextType::Guild,
                InteractionContextType::BotDm,
                InteractionContextType::PrivateChannel,
            ])
    }
}

impl CommandLocalize for StringBuilder {
    fn init_localize(
        global_name: impl Into<String>,
        global_description: impl Into<String>,
        local_names: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
        local_descriptions: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        Self::new(global_name, global_description)
            .name_localizations(local_names)
            .description_localizations(local_descriptions)
    }
}
