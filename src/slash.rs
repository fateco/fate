use twilight_model::application::command::Command;

#[derive(Debug)]
pub struct Slash(fn() -> Command);

inventory::collect!(Slash);

#[must_use]
pub fn get_commands() -> Vec<Command> {
    inventory::iter::<Slash>
        .into_iter()
        .map(|slash| (slash.0)())
        .collect()
}

// #[fate_internal_macro::command]
// fn test() -> Command {
//     twilight_util::builder::command::CommandBuilder::new(
//         "blep",
//         "Send a random adorable animal photo",
//         twilight_model::application::command::CommandType::ChatInput,
//     )
//     .build()
// }
