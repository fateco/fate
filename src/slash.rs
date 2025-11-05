use anyhow::Result;
use twilight_model::application::command::Command;

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
