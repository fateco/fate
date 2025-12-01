use twilight_model::{
    application::interaction::{
        Interaction, InteractionData,
        application_command::{CommandDataOption, CommandOptionValue},
        modal::ModalInteractionComponent,
    },
    id::{Id, marker::UserMarker},
};

pub trait InteractionDataHelper {
    fn get_id(&self) -> Option<&str>;
    fn get_user(&self) -> Option<(&Id<UserMarker>, &str, &str)>;
    fn get_user_id(&self) -> Option<&Id<UserMarker>>;
    fn get_username(&self) -> Option<&str>;
    fn get_locale(&self) -> &str;
    fn get_command_options(&self) -> Vec<&CommandDataOption>;
    fn get_command_v(&self, option_name: &str) -> Option<&CommandOptionValue>;
    fn get_command_v_str(&self, option_name: &str) -> Option<&str>;
    fn get_modal_components(&self) -> Vec<&ModalInteractionComponent>;
}

impl InteractionDataHelper for Interaction {
    fn get_id(&self) -> Option<&str> {
        self.data
            .as_ref()
            .and_then(|data| match data {
                InteractionData::ApplicationCommand(data) => Some(&data.name),
                InteractionData::MessageComponent(data) => Some(&data.custom_id),
                InteractionData::ModalSubmit(data) => Some(&data.custom_id),
                _ => None,
            })
            .map(String::as_str)
    }

    fn get_user(&self) -> Option<(&Id<UserMarker>, &str, &str)> {
        self.get_user_id()
            .and_then(|id| self.get_username().map(|name| (id, name)))
            .map(|(id, name)| (id, name, self.get_locale()))
    }

    fn get_user_id(&self) -> Option<&Id<UserMarker>> {
        self.user.as_ref().map(|user| &user.id)
    }

    fn get_username(&self) -> Option<&str> {
        self.user.as_ref().map(|user| user.name.as_str())
    }

    fn get_locale(&self) -> &str {
        self.locale.as_deref().unwrap_or("en-US")
    }

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

    fn get_modal_components(&self) -> Vec<&ModalInteractionComponent> {
        let top_components: Vec<&ModalInteractionComponent> = self
            .data
            .as_ref()
            .and_then(|data| match data {
                InteractionData::ModalSubmit(data) => Some(data.components.iter().collect()),
                _ => None,
            })
            .unwrap_or_default();

        top_components
            .into_iter()
            .map(|component| match component {
                ModalInteractionComponent::Label(label) => &label.component,
                _ => component,
            })
            .collect()
    }
}
