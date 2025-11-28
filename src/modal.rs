use twilight_model::{
    channel::message::{Component, component::ComponentType},
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use twilight_util::builder::message::LabelBuilder;
use worker::{Response, Result};

use crate::response::{internal_error, response};

pub struct Modal {
    title: String,
    components: Vec<Component>,
}

impl Modal {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            components: vec![],
        }
    }

    pub fn response(self, custom_id: impl Into<String>) -> Result<Response> {
        if self.components.is_empty() || self.components.len() > 5 {
            return internal_error();
        }
        response(&InteractionResponse {
            kind: InteractionResponseType::Modal,
            data: Some(InteractionResponseData {
                custom_id: Some(custom_id.into()),
                title: Some(self.title),
                components: Some(self.components),
                ..Default::default()
            }),
        })
    }

    pub fn row(
        mut self,
        label: impl Into<String>,
        description: impl Into<String>,
        component: Component,
    ) -> Self {
        if matches!(
            component.kind(),
            ComponentType::TextSelectMenu
                | ComponentType::TextInput
                | ComponentType::UserSelectMenu
                | ComponentType::RoleSelectMenu
                | ComponentType::MentionableSelectMenu
                | ComponentType::ChannelSelectMenu
                | ComponentType::TextDisplay
                | ComponentType::FileUpload
        ) {
            let mut label = label.into();
            label.truncate(45);
            let mut description = description.into();
            description.truncate(100);
            self.components.push(Component::Label(
                LabelBuilder::new(label, component)
                    .description(description)
                    .build(),
            ));
        }
        self
    }
}

pub mod input {
    use twilight_model::channel::message::component::{TextInput, TextInputStyle};

    pub struct TextInputBuilder {
        custom_id: String,
        id: Option<i32>,
        #[allow(deprecated)]
        label: Option<String>,
        max_length: Option<u16>,
        min_length: Option<u16>,
        placeholder: Option<String>,
        required: Option<bool>,
        style: TextInputStyle,
        value: Option<String>,
    }

    impl TextInputBuilder {
        pub fn new(custom_id: impl Into<String>) -> Self {
            Self {
                custom_id: custom_id.into(),
                id: None,
                #[allow(deprecated)]
                label: None,
                max_length: None,
                min_length: None,
                placeholder: None,
                required: Some(true),
                style: TextInputStyle::Short,
                value: None,
            }
        }

        pub const fn max_length(mut self, max_length: u16) -> Self {
            self.max_length = Some(max_length);
            self
        }

        pub const fn min_length(mut self, min_length: u16) -> Self {
            self.min_length = Some(min_length);
            self
        }

        pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
            self.placeholder = Some(placeholder.into());
            self
        }

        pub const fn required(mut self, required: bool) -> Self {
            self.required = Some(required);
            self
        }

        pub const fn paragraph(mut self) -> Self {
            self.style = TextInputStyle::Paragraph;
            self
        }

        pub fn value(mut self, value: impl Into<String>) -> Self {
            self.value = Some(value.into());
            self
        }

        pub fn build(self) -> TextInput {
            TextInput {
                custom_id: self.custom_id,
                id: self.id,
                #[allow(deprecated)]
                label: self.label,
                max_length: self.max_length,
                min_length: self.min_length,
                placeholder: self.placeholder,
                required: self.required,
                style: self.style,
                value: self.value,
            }
        }
    }
}
