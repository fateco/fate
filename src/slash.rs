use anyhow::Result;
use twilight_model::application::{
    command::{Command, CommandType},
    interaction::InteractionContextType,
};
use twilight_util::builder::command::{CommandBuilder, StringBuilder};
use unicode_segmentation::UnicodeSegmentation;

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
    fn init_localize<K: Into<String>, V: Into<String> + From<&'static str>>(
        name: &str,
        local_names: impl IntoIterator<Item = (K, V)> + Clone,
        local_descriptions: impl IntoIterator<Item = (K, V)> + Clone,
    ) -> Self;
}

pub trait StringChoiceBool {
    fn choice_bool(self) -> Self;
}

impl CommandLocalize for CommandBuilder {
    fn init_localize<K: Into<String>, V: Into<String> + From<&'static str>>(
        name: &str,
        local_names: impl IntoIterator<Item = (K, V)> + Clone,
        local_descriptions: impl IntoIterator<Item = (K, V)> + Clone,
    ) -> Self {
        Self::new(
            name.graphemes(false).take(32).collect::<String>(),
            local_descriptions
                .clone()
                .into_iter()
                .next()
                .map_or_else(|| "".into(), |v| v.1),
            CommandType::ChatInput,
        )
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
    fn init_localize<K: Into<String>, V: Into<String> + From<&'static str>>(
        name: &str,
        local_names: impl IntoIterator<Item = (K, V)> + Clone,
        local_descriptions: impl IntoIterator<Item = (K, V)> + Clone,
    ) -> Self {
        Self::new(
            name.graphemes(false).take(32).collect::<String>(),
            local_descriptions
                .clone()
                .into_iter()
                .next()
                .map_or_else(|| "".into(), |v| v.1),
        )
        .name_localizations(local_names)
        .description_localizations(local_descriptions)
    }
}

impl StringChoiceBool for StringBuilder {
    fn choice_bool(self) -> Self {
        self.choices([(t!("bool.yes"), "true"), (t!("bool.false"), "false")])
    }
}
