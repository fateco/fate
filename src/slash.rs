use twilight_model::application::command::Command;

#[derive(Debug)]
pub struct Slash(pub fn() -> Command);

inventory::collect!(Slash);

#[must_use]
pub fn get_commands() -> Vec<Command> {
    inventory::iter::<Slash>
        .into_iter()
        .map(|slash| (slash.0)())
        .collect()
}
