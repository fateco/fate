use twilight_model::application::interaction::{
    Interaction, InteractionData,
    application_command::{CommandDataOption, CommandOptionValue},
};

pub trait InteractionDataHelper {
    fn get_command_options(&self) -> Vec<&CommandDataOption>;
    fn get_command_v(&self, option_name: &str) -> Option<&CommandOptionValue>;
    fn get_command_v_str(&self, option_name: &str) -> Option<&str>;
}

impl InteractionDataHelper for Interaction {
    fn get_command_options(&self) -> Vec<&CommandDataOption> {
        self.data
            .as_ref()
            .and_then(|data| match data {
                InteractionData::ApplicationCommand(command_data) => {
                    Some(command_data.options.iter().collect())
                }
                _ => None,
            })
            .unwrap_or_default()
    }

    fn get_command_v(&self, option_name: &str) -> Option<&CommandOptionValue> {
        self.get_command_options()
            .iter()
            .find(|option| option.name == option_name)
            .map(|option| &option.value)
    }

    fn get_command_v_str(&self, option_name: &str) -> Option<&str> {
        self.get_command_v(option_name).and_then(|f| match f {
            CommandOptionValue::String(s) => Some(s.as_str()),
            _ => None,
        })
    }
}
